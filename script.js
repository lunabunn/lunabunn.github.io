for (let e of document.getElementsByClassName("tooltip")) {
    e.onclick = function (event) {
        if (this.copied) {
            return;
        }

        let tooltipText = this.getElementsByClassName("tooltiptext")[0];
        let e = event.toElement || event.relatedTarget;
        if (e == tooltipText) {
            return;
        }

        navigator?.clipboard?.writeText(tooltipText.innerHTML).then(() => {
            let content = tooltipText.innerHTML;
            switch (document.documentElement.lang) {
                case "ko": tooltipText.innerHTML = "복사됨!"; break;
                default: tooltipText.innerHTML = "Copied!";
            };
            this.copied = true;
            this.onmouseout = function (event) {
                let e = event.toElement || event.relatedTarget;
                if (e == this || e == tooltipText) {
                    return;
                }
                tooltipText.innerHTML = content;
                this.copied = false;
            };
        });
    };
}