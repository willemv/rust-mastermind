// Simple JavaScript Templating
// John Resig - http://ejohn.org/ - MIT Licensed
(function(exports){
    var cache = {};

    var load = function(template_name) {

        let template = document.getElementById(template_name).innerHTML;
        let function_body = "var p=[],print=function(){p.push.apply(p,arguments);};" +
         
            // Introduce the data as local variables using with(){}
            "with(obj){p.push('" +
            
            // Convert the template into pure JavaScript
            template
            .replace(/[\r\t\n]/g, " ")
            .split("<%").join("\t")
            .replace(/((^|%>)[^\t]*)'/g, "$1\r")
            .replace(/\t=(.*?)%>/g, "',$1,'")
            .split("\t").join("');")
            .split("%>").join("p.push('")
            .split("\r").join("\\'")
        + "');}return p.join('');";
      // Generate a reusable function that will serve as a template
      // generator (and which will be cached).
      return new Function("obj", function_body);
    };

    exports.tmpl = function tmpl(template, data) {
        var fn = cache[template] = cache[template] || load(template);
        
        // Provide some basic currying to the user
        return data ? fn( data ) : fn;
    };
  })(module.exports);