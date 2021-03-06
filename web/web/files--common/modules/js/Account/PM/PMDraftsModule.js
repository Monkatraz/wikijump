

Wikijump.modules.PMDraftsModule = {};

Wikijump.modules.PMDraftsModule.vars = {
	currentMessageId: null
}

Wikijump.modules.PMDraftsModule.listeners = {
	loadList: function(e, pageNo){
		var p = null;
		if(pageNo){p = {page: pageNo}};
		OZONE.ajax.requestModule("Account/PM/PMDraftsModule", p, Wikijump.modules.AccountMessagesModule.callbacks.setActionArea);
		if(e){	Wikijump.modules.AccountMessagesModule.utils.highlightTab(e);}
	},

	selectAll: function(e){
		var chs = YAHOO.util.Dom.getElementsByClassName("message-select");
		for(var i=0; i<chs.length; i++){
			chs[i].checked=true;
		}

	},

	removeSelected: function(e){
		var selected = new Array();
		var chs = YAHOO.util.Dom.getElementsByClassName("message-select");
		for(var i=0; i<chs.length; i++){
			if(chs[i].checked){
				selected.push(chs[i].id.replace(/message\-check\-/, ''));
			}
		}
		if(selected.length == 0){
			return;
		}
		var p = new Object();
		p.action = "PMAction";
		p.event = 'removeSelectedDrafts';
		p.selected = JSON.stringify(selected);
		OZONE.ajax.requestModule(null, p, Wikijump.modules.PMDraftsModule.callbacks.removeSelected);

	},
	removeDraftsMessage: function(e, messageId){
		Wikijump.modules.PMDraftsModule.vars.currentMessageId = messageId;
		var w = new OZONE.dialogs.ConfirmationDialog();
		w.content = "Are sure you want to remove this message?";
		w.buttons = ['cancel', 'remove message'];
		w.addButtonListener('cancel', w.close);
		w.addButtonListener('remove message', Wikijump.modules.PMDraftsModule.listeners.removeDraftsMessage2);
		w.focusButton = 'cancel';
		w.show();
	},

	removeDraftsMessage2: function(e, messageId){
		var p = new Object();
		p.action = "PMAction";
		p.event = 'removeDraftsMessage';
		p.message_id = Wikijump.modules.PMDraftsModule.vars.currentMessageId;
		OZONE.ajax.requestModule(null, p, Wikijump.modules.PMDraftsModule.callbacks.removeDraftsMessage);
	},
	editDraftMessage: function(e, messageId){
		var p = new Object();
		if(messageId){
			p.continueMessageId = messageId;
		}
		OZONE.ajax.requestModule("Account/PM/PMComposeModule", p, Wikijump.modules.PMDraftsModule.callbacks.editDraftMessage);

	}
}

Wikijump.modules.PMDraftsModule.callbacks = {
	removeSelected: function(r){
		Wikijump.modules.PMDraftsModule.listeners.loadList(null, 1);
	},
	removeDraftsMessage: function(r){
		if(r.status == 'ok'){
			var w = new OZONE.dialogs.SuccessBox();
			w.content = "The message has been removed.";
			w.show();

			if(r.messageId){
				setTimeout('Wikijump.modules.AccountMessagesModule.listeners.viewDraftsMessage("'+r.messageId+'")', 1000);
			}else{
				// return to inbox view
				setTimeout('draftsPage(1)');
			}
		}
	},
	editDraftMessage: function(r){
		if(!Wikijump.utils.handleError(r)) {return;}
		if(r.toUserId){
			Wikijump.modules.AccountMessagesModule.vars.toUserId = r.toUserId;
			Wikijump.modules.AccountMessagesModule.vars.toUserName = r.toUserName;
		}else{
			Wikijump.modules.AccountMessagesModule.vars.toUserId = null;
			Wikijump.modules.AccountMessagesModule.vars.toUserName = null;
		}
		$("pm-action-area").innerHTML = r.body;
		// format dates
		OZONE.utils.formatDates($("pm-action-area"));

		var tp = $("account-top-tabs");
		var as = tp.getElementsByTagName('a');
		for(var i=0; i<as.length; i++){
			YAHOO.util.Dom.removeClass(as[i], "active");
		}
		var curr = as.item(3);
		YAHOO.util.Dom.addClass(curr, "active");

		YAHOO.util.Event.addListener("pm-compose-cancel-button", "click", Wikijump.modules.AccountMessagesModule.listeners.drafts);
	}
}

Wikijump.modules.PMDraftsModule.init = function(){

}
