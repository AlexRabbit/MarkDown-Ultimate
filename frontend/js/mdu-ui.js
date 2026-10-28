window.mduStartTour = function (stepsJson) {
  if (!window.driver || !window.driver.js || !window.driver.js.driver) {
    console.warn("Driver.js not loaded");
    return;
  }
  const steps = JSON.parse(stepsJson);
  const driver = window.driver.js.driver;
  const drv = driver({
    showProgress: true,
    animate: true,
    allowClose: true,
    overlayOpacity: 0.45,
    popoverClass: "mdu-driver-popover",
    steps: steps.map(function (s) {
      return {
        element: s.element,
        popover: {
          title: s.title,
          description: s.description,
          side: s.side || "bottom",
          align: s.align || "start",
        },
      };
    }),
  });
  drv.drive();
};

window.mduRenderMath = function (rootSelector) {
  if (!window.renderMathInElement) return;
  var el = document.querySelector(rootSelector);
  if (!el) return;
  window.renderMathInElement(el, {
    delimiters: [
      { left: "$$", right: "$$", display: true },
      { left: "$", right: "$", display: false },
    ],
    throwOnError: false,
  });
};
