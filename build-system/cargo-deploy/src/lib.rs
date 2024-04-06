#![allow(async_fn_in_trait)]

use std::{
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
};

use futures::StreamExt;
use ssh::Session;
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
};
use tokio_stream::wrappers::ReadDirStream;

pub mod ssh;

const LIB_DEPLOY_DIR: &str = "/usr/local/frc/third-party/lib";

pub struct TeamNumber(u32);

impl TeamNumber {
    pub fn new(num: u32) -> Self {
        assert!(num < 10000);

        Self(num)
    }
}

async fn find_rio_address(team_number: TeamNumber) -> Option<IpAddr> {
    let addr = IpAddr::V4(Ipv4Addr::new(
        10,
        (team_number.0 / 100) as u8,
        (team_number.0 % 100) as u8,
        2,
    ));

    if check_addr(addr).await {
        return Some(addr);
    }

    None
}

async fn check_addr(addr: IpAddr) -> bool {
    let payload = [0; 8];

    surge_ping::ping(addr, &payload).await.is_ok()
}

pub async fn create_target(team_number: TeamNumber) -> Option<Session> {
    let addr = find_rio_address(team_number).await?;

    Session::connect(addr).await.ok()
}

pub trait Action {
    async fn execute(&mut self, target: &Session);
}

pub struct ProgramKill;

impl Action for ProgramKill {
    async fn execute(&mut self, target: &Session) {
        if String::from_utf8(
            target
                .call("md5sum /usr/local/frc/bin/frcKillRobot.sh")
                .await
                .unwrap(),
        )
        .unwrap()
        .trim()
            != "001696b0412c36f7be0c868ad493d16a  /usr/local/frc/bin/frcKillRobot.sh"
        {
            panic!("Kill robot script not correct, please deploy using official method");
        }

        target
            .call(
                "cd /home/lvuser; . /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t 2> /dev/null"
            )
            .await.unwrap();
    }
}

pub struct DeployStartCommand;

impl Action for DeployStartCommand {
    async fn execute(&mut self, target: &Session) {
        target.call("cd /home/lvuser; echo '\"/home/lvuser/frcUserProgram\" ' > /home/lvuser/robotCommand").await.unwrap();
        target.call("cd /home/lvuser; chmod +x /home/lvuser/robotCommand; chown lvuser /home/lvuser/robotCommand").await.unwrap();
    }
}

pub struct DeployCode<'a> {
    pub local: &'a Path,
}

impl<'a> Action for DeployCode<'a> {
    async fn execute(&mut self, target: &Session) {
        let sftp = target.sftp().await.unwrap();

        let mut local_file = File::open(self.local).await.unwrap();
        sftp.create("/home/lvuser/frcUserProgram").await.unwrap();

        let mut buf = vec![];

        local_file.read_to_end(&mut buf).await.unwrap();

        sftp.write("/home/lvuser/frcUserProgram", &buf)
            .await
            .unwrap();

        target.call("cd /home/lvuser; chmod +x \"/home/lvuser/frcUserProgram\"; chown lvuser \"/home/lvuser/frcUserProgram\"").await.unwrap();
        target
            .call("cd /home/lvuser; setcap cap_sys_nice+eip \"/home/lvuser/frcUserProgram\"")
            .await
            .unwrap();
    }
}

pub struct StartProgram;

impl Action for StartProgram {
    async fn execute(&mut self, target: &Session) {
        target.call("cd /home/lvuser; sync; /usr/local/natinst/bin/nirtcfg --file=/etc/natinst/share/ni-rt.ini --get section=systemsettings,token=NoApp.enabled,value=unknown; . /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t -r 2> /dev/null").await.unwrap();
    }
}

pub struct DeployLibraries<'a> {
    pub libs: &'a Path,
}

impl<'a> Action for DeployLibraries<'a> {
    async fn execute(&mut self, target: &Session) {
        let sftp = target.sftp().await.unwrap();

        let _ = sftp.create_dir(LIB_DEPLOY_DIR).await;

        ReadDirStream::new(fs::read_dir(self.libs).await.unwrap())
            .for_each_concurrent(Some(5), |entry| async {
                let entry = entry.unwrap();
                if !entry.file_type().await.unwrap().is_file() {
                    return;
                }

                let mut local_file = File::open(entry.path()).await.unwrap();

                let remote_path = PathBuf::from(LIB_DEPLOY_DIR)
                    .join(entry.path().file_name().unwrap().to_str().unwrap());

                let mut buf = vec![];

                local_file.read_to_end(&mut buf).await.unwrap();

                if target
                    .call(format!(
                        "md5sum -cs <<< '{:x}  {}'",
                        md5::compute(&buf),
                        remote_path.to_str().unwrap()
                    ))
                    .await
                    .is_ok()
                {
                    println!("Did not redeploy {}", entry.path().to_str().unwrap());
                    return;
                }

                sftp.create(remote_path.to_str().unwrap()).await.unwrap();

                sftp.write(remote_path.to_str().unwrap(), &buf)
                    .await
                    .unwrap();

                println!("Deployed {}", entry.path().to_str().unwrap());
            })
            .await;

        target.call("cd /home/lvuser; chmod -R 777 \"/usr/local/frc/third-party/lib\" || true; chown -R lvuser:ni \"/usr/local/frc/third-party/lib\"").await.unwrap();
        target.call("cd /home/lvuser; ldconfig").await.unwrap();
    }
}
