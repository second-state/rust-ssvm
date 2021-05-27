(function() {var implementors = {};
implementors["num_bigint"] = [{"text":"impl&lt;T&gt; Sum&lt;T&gt; for BigInt <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BigInt: Add&lt;T, Output = BigInt&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Sum&lt;T&gt; for BigUint <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BigUint: Add&lt;T, Output = BigUint&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;Num + Clone&gt; Sum&lt;Complex&lt;T&gt;&gt; for Complex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;'a + Num + Clone&gt; Sum&lt;&amp;'a Complex&lt;T&gt;&gt; for Complex&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Integer + Clone&gt; Sum&lt;Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Integer + Clone&gt; Sum&lt;&amp;'a Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()