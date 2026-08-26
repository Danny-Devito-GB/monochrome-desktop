// WebKitGtk does not support the reload shortcut (Ctrl+R or F5) by default so we need to implement it manually.
document.addEventListener('keydown', (event) => {
  if (event.key === 'F5' || (event.ctrlKey && event.key === 'r')) {
    window.location.reload();
  }
});