(function() {
    var type_impls = Object.fromEntries([["math",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Controller-for-Velocity%3CC%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/math/lib.rs.html#189-203\">source</a><a href=\"#impl-Controller-for-Velocity%3CC%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C: <a class=\"trait\" href=\"math/trait.Controller.html\" title=\"trait math::Controller\">Controller</a>&lt;State = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>&gt;&gt; <a class=\"trait\" href=\"math/trait.Controller.html\" title=\"trait math::Controller\">Controller</a> for <a class=\"struct\" href=\"math/struct.Velocity.html\" title=\"struct math::Velocity\">Velocity</a>&lt;C&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/math/lib.rs.html#190\">source</a><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"math/trait.Controller.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = &lt;C as <a class=\"trait\" href=\"math/trait.Controller.html\" title=\"trait math::Controller\">Controller</a>&gt;::<a class=\"associatedtype\" href=\"math/trait.Controller.html#associatedtype.Output\" title=\"type math::Controller::Output\">Output</a></h4></section><section id=\"associatedtype.State\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/math/lib.rs.html#191\">source</a><a href=\"#associatedtype.State\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"math/trait.Controller.html#associatedtype.State\" class=\"associatedtype\">State</a> = <a class=\"struct\" href=\"math/struct.State.html\" title=\"struct math::State\">State</a></h4></section><section id=\"method.calculate_with_time\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/math/lib.rs.html#194-202\">source</a><a href=\"#method.calculate_with_time\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"math/trait.Controller.html#tymethod.calculate_with_time\" class=\"fn\">calculate_with_time</a>(\n    &amp;mut self,\n    current: &amp;<a class=\"struct\" href=\"math/struct.State.html\" title=\"struct math::State\">State</a>,\n    target: &amp;<a class=\"struct\" href=\"math/struct.State.html\" title=\"struct math::State\">State</a>,\n    time: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>,\n) -&gt; C::<a class=\"associatedtype\" href=\"math/trait.Controller.html#associatedtype.Output\" title=\"type math::Controller::Output\">Output</a></h4></section><section id=\"method.calculate\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/math/lib.rs.html#89-91\">source</a><a href=\"#method.calculate\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"math/trait.Controller.html#method.calculate\" class=\"fn\">calculate</a>(\n    &amp;mut self,\n    current: &amp;Self::<a class=\"associatedtype\" href=\"math/trait.Controller.html#associatedtype.State\" title=\"type math::Controller::State\">State</a>,\n    target: &amp;Self::<a class=\"associatedtype\" href=\"math/trait.Controller.html#associatedtype.State\" title=\"type math::Controller::State\">State</a>,\n) -&gt; Self::<a class=\"associatedtype\" href=\"math/trait.Controller.html#associatedtype.Output\" title=\"type math::Controller::Output\">Output</a></h4></section></div></details>","Controller","math::feedforward::Velocity","math::feedforward::Acceleration"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-Velocity%3CC%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/math/lib.rs.html#223-227\">source</a><a href=\"#impl-Default-for-Velocity%3CC%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"math/struct.Velocity.html\" title=\"struct math::Velocity\">Velocity</a>&lt;C&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/math/lib.rs.html#224-226\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","math::feedforward::Velocity","math::feedforward::Acceleration"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[5121]}