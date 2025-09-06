htmx.defineExtension("size-limit", {
  onEvent: function (name, evt) {
    if (name !== "htmx:beforeRequest") return;
    const el = evt.target;
    const maxSize =
      (parseFloat(el.getAttribute("data-max-size")) || 0.5) * 1024 * 1024;
    const fileInput = el.querySelector('input[type="file"]');
    if (!fileInput?.files.length) return;
    const file = fileInput.files[0];
    if (file.size > maxSize) {
      evt.preventDefault();
      htmx.ajax("POST", "/htmx/alert", {
        target: "#hxtoast",
        swap: "beforeend",
        values: {
          message: `File too large (${(file.size / 1024 / 1024).toFixed(1)}MB, max ${(maxSize / 1024 / 1024).toFixed(1)}MB)`,
          level: "Error",
        },
      });
    }
  },
});
