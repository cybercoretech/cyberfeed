import QtQuick
import Quickshell
import qs.Commons
import qs.Ui

Item {
    id: root

    property var bar: null
    moduleName: "cybercore.cyberfeed"

    readonly property string barIcon: "📰"

    function launch() {
        Quickshell.execDetached(["ghostty", "-e", "/home/raven/Devspace/Cybercore/tooling/cyberfeed/target/release/cyberfeed"])
    }

    implicitWidth: button.implicitWidth
    implicitHeight: button.implicitHeight

    WidgetButton {
        id: button

        anchors.fill: parent
        bar: root.bar
        text: root.barIcon
        tooltipText: "Cyberfeed RSS Reader"

        onPressed: function(buttonCode) {
            if (buttonCode === Qt.LeftButton)
                root.launch()
        }
    }
}
