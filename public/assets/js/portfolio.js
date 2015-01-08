(function(window, ns, undefined){

    ns.Portfolio = new Class({
        initialize: function(container) {
            this.container = window.document.id(container);
            this.items = this.container.getChildren();

            for (i = 0; i < this.items.length; i++) {
                var item = this.items[i];

                // item data
                item.portfolio = this;
                item.index = i;

                // click event
                item.addEvent("click", function() {
                    this.portfolio.showItem(this.index);
                });
            }
        },

        showItem: function(index) {
            if (index < this.items.length) {
                for (i = 0; i < this.items.length; i++) {
                    this.items[i].removeClass("active");
                }

                var item = this.items[index];

                // mark as active
                item.addClass("active");

                // scroll to item
                var scroll = new window.Fx.Scroll(this.container, {duration: 200});
                scroll.start(item.offsetLeft - (item.offsetWidth / 2), 0);

                this.currentIndex = index;
            }
        }
    });

})(window, window.web = window.web || {});