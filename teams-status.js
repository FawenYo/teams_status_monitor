const i18n = {
    zh_TW: {
        title: "會議狀態",
        duration: "會議時長",
        in_meeting: "正在開會",
        not_in_meeting: "未開會",
    },
    en: {
        title: "Meeting Status",
        duration: "Meeting Duration",
        in_meeting: "Busy",
        not_in_meeting: "Free",
    },
};
const strings = i18n[Device.locale()] ?? i18n["en"];

const url = `https://tsmb.fawenyo.pp.ua/api/monitor/teams?user=FawenYo`;
const req = new Request(url);
const res = await req.loadJSON();

if (!res.data) {
    throw new Error("Invalid response");
}

let widget = new ListWidget();
widget.refreshAfterDate = new Date(Date.now() + 1000 * 30); // Update every 30 seconds
widget.backgroundColor = Color.dynamic(Color.white(), Color.darkGray());

const topWidgetStack = widget.addStack();
const title = topWidgetStack.addText(strings.title);
title.font = Font.regularSystemFont(12);
topWidgetStack.addSpacer();
const clockSymbol = SFSymbol.named("clock.badge");
const icon = topWidgetStack.addImage(clockSymbol.image);
icon.imageSize = new Size(16, 16);
widget.addSpacer();

const resinTextStack = widget.addStack();
resinTextStack.bottomAlignContent();
const currentMeetText = resinTextStack.addText(
    `${res.data.in_meeting ? strings.in_meeting : strings.not_in_meeting}`
);
if (res.data.in_meeting) {
    currentMeetText.textColor = Color.red();
} else {
    currentMeetText.textColor = Color.green();
}
currentMeetText.font = Font.boldRoundedSystemFont(64);
currentMeetText.minimumScaleFactor = 0.5;
widget.addSpacer();

const leftText = widget.addText(strings.duration);
leftText.font = Font.regularSystemFont(10);
const meetingDurationText = widget.addText(`${res.data.meeting_duration}`);
meetingDurationText.font = Font.regularSystemFont(16);
meetingDurationText.minimumScaleFactor = 0.5;

Script.setWidget(widget);
Script.complete();
