(() => {

    const _addEventListener = EventTarget.prototype.addEventListener;

    let myWasmReady = false;

    const myDelayedEvents = [];

    EventTarget.prototype.addEventListener = function (type, fn, ...others) {
        if (type === "DOMContentLoaded" || type === "load") {
            return _addEventListener.call(this, type, (...args) => {
                if (myWasmReady) {
                    fn(...args);
                } else {
                    myDelayedEvents.push({
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
        myWasmReady = true;

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
