(function () {
    if (window.__monochromeIntentOverrideInjected) {
        return;
    }
    window.__monochromeIntentOverrideInjected = true;

    // Force outgoing track intents to download instead of streaming.
    function rewrite(url) {
        try {
            if (typeof url === 'string' && url.includes('intent=stream')) {
                return url.replace(/intent=stream/g, 'intent=download');
            }
        } catch (error) { }
        return url;
    }

    const originalFetch = window.fetch;
    window.fetch = function (input, init) {
        return originalFetch.call(this, rewrite(input), init);
    };
})();