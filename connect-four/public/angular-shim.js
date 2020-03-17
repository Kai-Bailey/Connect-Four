(() => {

    const _addEventListener = EventTarget.prototype.addEventListener;

    let wasmReady = false;

    const myDelayedEvents = [];

    EventTarget.prototype.addEventListener = function (type, fn, ...others) {
        if (type === "DOMContentLoaded" || type === "load") {
            return _addEventListener.call(this, type, (...args) => {
                if (wasmReady) {
                    fn(...args);
                } else {
                    myDelayedEvents.push({
                        event: type,
                        fn: fn,
                        args: args,
                    });
                }
            }, ...others);
        } else {
            return _addEventListener.call(this, type, fn, ...others);
        }
    };

    window.wasmReady = () => {
        wasmReady = true;

        for (entry of myDelayedEvents) {
            try {
                entry.fn(...entry.args);
            } catch (err) {
                console.error(err);
            }
        }

        myDelayedEvents.length = 0;
    };

})();
