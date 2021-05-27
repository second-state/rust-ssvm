(function() {var implementors = {};
implementors["num_bigint"] = [{"text":"impl PartialOrd&lt;Sign&gt; for Sign","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;BigUint&gt; for BigUint","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer&gt; PartialOrd&lt;Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["vec_map"] = [{"text":"impl&lt;V:&nbsp;PartialOrd&gt; PartialOrd&lt;VecMap&lt;V&gt;&gt; for VecMap&lt;V&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()