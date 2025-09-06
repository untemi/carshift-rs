htmx.defineExtension("null-empty", {
  encodeParameters: function (xhr, parameters, elt) {
    for (const [name, value] of Array.from(parameters.entries())) {
      if (value === "") parameters.delete(name);
    }

    return null;
  },
});
