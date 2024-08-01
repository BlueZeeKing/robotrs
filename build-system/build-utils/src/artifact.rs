use std::{env, fmt::Display, io::Cursor};

use bytes::Bytes;
use thiserror::Error;
use zip::{read::ZipFile, ZipArchive};

use crate::zip::extract_libs;

pub fn is_robot() -> bool {
    get_platform() == "linuxathena"
}

fn get_platform() -> String {
    let os = match env::var("CARGO_CFG_TARGET_OS")
        .expect("No target OS set (cargo should have set this)")
        .as_str()
    {
        "windows" => "windows",
        "linux" => "linux",
        "macos" => "osx",
        os => panic!("Unknown/unsupported os: {}", os),
    };

    let arch = match env::var("CARGO_CFG_TARGET_ARCH")
        .expect("No target OS set (cargo should have set this)")
        .as_str()
    {
        "arm" => "athena",
        "aarch64" => "arm64",
        "x86_64" => "x86-64",
        arch => panic!("Unknown/unsupported target arch: {}", arch),
    };

    let arch = if os == "osx" { "universal" } else { arch };

    format!("{}{}", os, arch)
}

#[derive(Debug)]
pub struct Artifact {
    group_id: String,
    artifact_id: String,
    version: String,
    maven_url: String,
    lib_name: Option<String>,
    headers: bool,
    should_deploy: bool,
    robot_only: bool,
}

impl Artifact {
    fn get_url(&self, platform: &str) -> String {
        format!(
            "{}{}/{}/{}/{}-{}-{}.zip",
            self.maven_url,
            self.group_id.replace('.', "/"),
            self.artifact_id,
            self.version,
            self.artifact_id,
            self.version,
            platform
        )
    }

    pub fn get_header_url(&self) -> String {
        self.get_url("headers")
    }

    pub fn get_lib_url(&self) -> String {
        self.get_url(&get_platform())
    }

    pub fn get_lib_name(&self) -> Option<&str> {
        self.lib_name.as_deref()
    }

    pub fn find_lib_in_zip<'a>(
        &self,
        archive: &'a mut ZipArchive<Cursor<Bytes>>,
    ) -> anyhow::Result<ZipFile<'a>> {
        let (_, file_number) = extract_libs(archive)?
            .into_iter()
            .map(|val| dbg!(val))
            .find(|(name, _)| name == self.get_lib_name().unwrap())
            .unwrap();

        Ok(archive.by_index(file_number)?)
    }

    pub fn has_headers(&self) -> bool {
        self.headers
    }

    pub fn should_deploy(&self) -> bool {
        self.should_deploy
    }

    pub fn is_robot_only(&self) -> bool {
        self.robot_only
    }
}

impl Artifact {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

pub struct Builder {
    group_id: Option<String>,
    artifact_id: Option<String>,
    version: Option<String>,
    maven_url: Option<String>,
    artifact_name: Option<String>,
    lib_name: Option<String>,
    headers: bool,
    should_deploy: bool,
    robot_only: bool,
}

#[derive(Clone, Copy)]
pub enum Target {
    Headers,
    RoboRio,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Headers => write!(f, "headers"),
            Target::RoboRio => write!(f, "linuxathena"),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing group id (e.g. `com.google`)")]
    MissingGroupId,
    #[error("Missing artifact id (e.g. `gson`)")]
    MissingArtifactId,
    #[error("Missing version (e.g. `2.10.1`)")]
    MissingVersion,
    #[error("Missing maven url (e.g. `https://repo.maven.apache.org/maven2/`)")]
    MissingMavenUrl,
    #[error("Missing target or artifact name (only one is needed)")]
    MissingArtifactName,
}

impl Builder {
    fn new() -> Self {
        Self {
            group_id: None,
            artifact_id: None,
            version: None,
            maven_url: None,
            artifact_name: None,
            lib_name: None,
            headers: true,
            should_deploy: true,
            robot_only: false,
        }
    }

    pub fn build(&self) -> Result<Artifact, Error> {
        let group_id = self.group_id.to_owned().ok_or(Error::MissingGroupId)?;
        let artifact_id = self
            .artifact_id
            .to_owned()
            .ok_or(Error::MissingArtifactId)?;
        let version = self.version.to_owned().ok_or(Error::MissingVersion)?;
        let maven_url = self.maven_url.to_owned().ok_or(Error::MissingMavenUrl)?;

        Ok(Artifact {
            group_id,
            artifact_id,
            version,
            maven_url,
            lib_name: self.lib_name.to_owned(),
            headers: self.headers,
            should_deploy: self.should_deploy,
            robot_only: self.robot_only,
        })
    }

    pub fn group_id(&mut self, group_id: String) -> &mut Self {
        self.group_id = Some(group_id);

        self
    }

    pub fn artifact_id(&mut self, artifact_id: String) -> &mut Self {
        self.artifact_id = Some(artifact_id);

        self
    }

    pub fn version(&mut self, version: String) -> &mut Self {
        self.version = Some(version);

        self
    }

    pub fn maven_url(&mut self, maven_url: String) -> &mut Self {
        self.maven_url = Some(maven_url);

        self
    }

    pub fn artifact_name(&mut self, artifact_name: String) -> &mut Self {
        self.artifact_name = Some(artifact_name);

        self
    }

    pub fn lib_name(&mut self, lib_name: String) -> &mut Self {
        self.lib_name = Some(lib_name);

        self
    }

    pub fn no_headers(&mut self) -> &mut Self {
        self.headers = false;

        self
    }

    pub fn no_deploy(&mut self) -> &mut Self {
        self.should_deploy = false;

        self
    }

    pub fn robot_only(&mut self) -> &mut Self {
        self.robot_only = true;

        self
    }
}
