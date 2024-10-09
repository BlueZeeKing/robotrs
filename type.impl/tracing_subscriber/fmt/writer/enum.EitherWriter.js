(function() {
    var type_impls = Object.fromEntries([["tracing_subscriber",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-EitherWriter%3CA,+B%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#impl-Clone-for-EitherWriter%3CA,+B%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"EitherWriter&lt;A, B&gt;\">ⓘ</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","tracing_subscriber::fmt::writer::OptionalWriter"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-EitherWriter%3CA,+B%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#impl-Debug-for-EitherWriter%3CA,+B%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","tracing_subscriber::fmt::writer::OptionalWriter"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-EitherWriter%3CA,+B%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#impl-PartialEq-for-EitherWriter%3CA,+B%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>, B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#261\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","tracing_subscriber::fmt::writer::OptionalWriter"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Write-for-EitherWriter%3CA,+B%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#832-876\">source</a><a href=\"#impl-Write-for-EitherWriter%3CA,+B%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt;<div class=\"where\">where\n    A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.write\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#838-843\">source</a><a href=\"#method.write\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write\" class=\"fn\">write</a>(&amp;mut self, buf: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class='docblock'>Writes a buffer into this writer, returning how many bytes were written. <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.flush\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#846-851\">source</a><a href=\"#method.flush\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush\" class=\"fn\">flush</a>(&amp;mut self) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;</h4></section></summary><div class='docblock'>Flushes this output stream, ensuring that all intermediately buffered\ncontents reach their destination. <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_vectored\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#854-859\">source</a><a href=\"#method.write_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored\" class=\"fn\">write_vectored</a>(&amp;mut self, bufs: &amp;[<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html\" title=\"struct std::io::IoSlice\">IoSlice</a>&lt;'_&gt;]) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class='docblock'>Like <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write\" title=\"method std::io::Write::write\"><code>write</code></a>, except that it writes from a slice of buffers. <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_all\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#862-867\">source</a><a href=\"#method.write_all\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all\" class=\"fn\">write_all</a>(&amp;mut self, buf: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;</h4></section></summary><div class='docblock'>Attempts to write an entire buffer into this writer. <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#870-875\">source</a><a href=\"#method.write_fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt\" class=\"fn\">write_fmt</a>(&amp;mut self, fmt: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html\" title=\"struct core::fmt::Arguments\">Arguments</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;</h4></section></summary><div class='docblock'>Writes a formatted string into this writer, returning any error\nencountered. <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_write_vectored\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1641\">source</a><a href=\"#method.is_write_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored\" class=\"fn\">is_write_vectored</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>can_vector</code>&nbsp;<a href=\"https://github.com/tokio-rs/tracing/issues/69941\">#69941</a>)</span></div></span><div class='docblock'>Determines if this <code>Write</code>r has an efficient <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored\" title=\"method std::io::Write::write_vectored\"><code>write_vectored</code></a>\nimplementation. <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_all_vectored\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1765\">source</a><a href=\"#method.write_all_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored\" class=\"fn\">write_all_vectored</a>(&amp;mut self, bufs: &amp;mut [<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html\" title=\"struct std::io::IoSlice\">IoSlice</a>&lt;'_&gt;]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>write_all_vectored</code>&nbsp;<a href=\"https://github.com/tokio-rs/tracing/issues/70436\">#70436</a>)</span></div></span><div class='docblock'>Attempts to write multiple buffers into this writer. <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.by_ref\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1878-1880\">source</a></span><a href=\"#method.by_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref\" class=\"fn\">by_ref</a>(&amp;mut self) -&gt; &amp;mut Self<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Creates a “by reference” adapter for this instance of <code>Write</code>. <a href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref\">Read more</a></div></details></div></details>","Write","tracing_subscriber::fmt::writer::OptionalWriter"],["<section id=\"impl-Copy-for-EitherWriter%3CA,+B%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#impl-Copy-for-EitherWriter%3CA,+B%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt;</h3></section>","Copy","tracing_subscriber::fmt::writer::OptionalWriter"],["<section id=\"impl-Eq-for-EitherWriter%3CA,+B%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#impl-Eq-for-EitherWriter%3CA,+B%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>, B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt;</h3></section>","Eq","tracing_subscriber::fmt::writer::OptionalWriter"],["<section id=\"impl-StructuralPartialEq-for-EitherWriter%3CA,+B%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tracing_subscriber/fmt/writer.rs.html#552\">source</a><a href=\"#impl-StructuralPartialEq-for-EitherWriter%3CA,+B%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"enum\" href=\"tracing_subscriber/fmt/writer/enum.EitherWriter.html\" title=\"enum tracing_subscriber::fmt::writer::EitherWriter\">EitherWriter</a>&lt;A, B&gt;</h3></section>","StructuralPartialEq","tracing_subscriber::fmt::writer::OptionalWriter"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[20094]}