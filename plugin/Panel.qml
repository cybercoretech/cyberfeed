import QtQuick
import Quickshell
import qs.Commons
import qs.Ui

Panel {
    id: root
    manageIpc: false

    function open() { root.controller.show() }
    function close() { root.controller.hide() }

    KeyboardPanel {
        id: panel
        anchorItem: root.anchorItem
        owner: root.hostWidget || root
        bar: root.bar
        open: root.opened
        focusTarget: keyCatcher
        contentWidth: panel.fittedContentWidth(Style.space(320))
        contentHeight: panel.fittedContentHeight(content.implicitHeight)

        PanelKeyCatcher {
            id: keyCatcher
            anchors.fill: parent
            onCloseRequested: root.close()

            Column {
                id: content
                width: parent.width
                spacing: Style.space(12)

                Text {
                    width: parent.width
                    text: "Cyberfeed RSS Reader"
                    color: root.barForeground
                    font.family: root.bar ? root.bar.fontFamily : Style.font.family
                    font.pixelSize: Style.font.subtitle
                    font.bold: true
                }

                WidgetButton {
                    width: parent.width
                    text: "🚀 Launch Cyberfeed TUI"
                    onPressed: function() {
                        Quickshell.execDetached(["ghostty", "-e", "/home/raven/Devspace/Cybercore/tooling/cyberfeed/target/release/cyberfeed"])
                        root.close()
                    }
                }
            }
        }
    }
}
