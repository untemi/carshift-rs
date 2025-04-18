 htmx.defineExtension("null-empty", {
   encodeParameters: function (xhr, parameters, elt) {
     for (const key of parameters.keys()) {
       if (parameters.get(key) == "") {
         parameters.delete(key);
       }
     }
     return null;
   },
 });
