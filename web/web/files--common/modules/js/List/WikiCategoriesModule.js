

Wikijump.modules.WikiCategoriesModule = {};

Wikijump.modules.WikiCategoriesModule.listeners = {
	toggleListPages: function(event, categoryId){
		var ldId = 'category-pages-'+categoryId;
		var d = $(ldId);
		if(d.style.display == "block"){
			d.style.display = "none";
			$('category-pages-toggler-'+categoryId).innerHTML = "+ list pages";
		}else{
			if(d.innerHTML==""){
				var p = new Object();
				p.category_id = categoryId;
				d.style.display = "block";
				d.innerHTML = '<div class="wait-block">loading page list...</div>';
				OZONE.ajax.requestModule('List/WikiCategoriesPageListModule', p, Wikijump.modules.WikiCategoriesModule.callbacks.listPages);
			}else{
				d.style.display = "block";
			}
			$('category-pages-toggler-'+categoryId).innerHTML = "- hide pages";
		}
	}
}

Wikijump.modules.WikiCategoriesModule.callbacks = {
	listPages: function(r){
		var ldId = 'category-pages-'+r.categoryId;
		var d = $(ldId);
		d.innerHTML = r.body;

	}
}
