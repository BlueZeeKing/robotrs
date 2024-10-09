(function() {
    var implementors = Object.fromEntries([["bitvec",[["impl&lt;M, T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/ptr/struct.BitPtr.html\" title=\"struct bitvec::ptr::BitPtr\">BitPtr</a>&lt;M, T, O&gt;<div class=\"where\">where\n    M: <a class=\"trait\" href=\"bitvec/ptr/trait.Mutability.html\" title=\"trait bitvec::ptr::Mutability\">Mutability</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,</div>"],["impl&lt;M, T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/ptr/struct.BitRef.html\" title=\"struct bitvec::ptr::BitRef\">BitRef</a>&lt;'_, M, T, O&gt;<div class=\"where\">where\n    M: <a class=\"trait\" href=\"bitvec/ptr/trait.Mutability.html\" title=\"trait bitvec::ptr::Mutability\">Mutability</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,</div>"],["impl&lt;T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/boxed/struct.BitBox.html\" title=\"struct bitvec::boxed::BitBox\">BitBox</a>&lt;T, O&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/slice/struct.BitSlice.html\" title=\"struct bitvec::slice::BitSlice\">BitSlice</a>&lt;T, O&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,</div>"],["impl&lt;T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/vec/struct.BitVec.html\" title=\"struct bitvec::vec::BitVec\">BitVec</a>&lt;T, O&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"]]],["nalgebra",[["impl&lt;T, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, R, C, S&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>,\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, R, C&gt;,</div>"]]],["wyz",[["impl&lt;M, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/comu/struct.Address.html\" title=\"struct wyz::comu::Address\">Address</a>&lt;M, T&gt;<div class=\"where\">where\n    M: <a class=\"trait\" href=\"wyz/comu/trait.Mutability.html\" title=\"trait wyz::comu::Mutability\">Mutability</a>,\n    T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html\" title=\"trait core::fmt::Binary\">Binary</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtBinary.html\" title=\"struct wyz::fmt::FmtBinary\">FmtBinary</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtDisplay.html\" title=\"struct wyz::fmt::FmtDisplay\">FmtDisplay</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html\" title=\"trait core::fmt::LowerExp\">LowerExp</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtLowerExp.html\" title=\"struct wyz::fmt::FmtLowerExp\">FmtLowerExp</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html\" title=\"trait core::fmt::LowerHex\">LowerHex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtLowerHex.html\" title=\"struct wyz::fmt::FmtLowerHex\">FmtLowerHex</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html\" title=\"trait core::fmt::Octal\">Octal</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtOctal.html\" title=\"struct wyz::fmt::FmtOctal\">FmtOctal</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtPointer.html\" title=\"struct wyz::fmt::FmtPointer\">FmtPointer</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtUpperExp.html\" title=\"struct wyz::fmt::FmtUpperExp\">FmtUpperExp</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtUpperHex.html\" title=\"struct wyz::fmt::FmtUpperHex\">FmtUpperHex</a>&lt;T&gt;"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[3087,1013,4921]}