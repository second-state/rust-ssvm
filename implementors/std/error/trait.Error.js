(function() {var implementors = {};
implementors["clap"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["hex"] = [{"text":"impl Error for FromHexError","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl Error for ParseBigIntError","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Error for TryFromBigIntError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Debug,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;E:&nbsp;Error&gt; Error for ParseComplexError&lt;E&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl Error for ParseRatioError","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()