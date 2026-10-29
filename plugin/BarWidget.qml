import QtQuick
import Quickshell
import qs.Commons
import qs.Ui

Item {
    id: root

    property var bar: null

    readonly property string barIcon: "📰"

    function launch() {
        Quickshell.execDetached(["ghostty", "-e", "/home/raven/.cargo-target/release/cyberfeed"])
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
