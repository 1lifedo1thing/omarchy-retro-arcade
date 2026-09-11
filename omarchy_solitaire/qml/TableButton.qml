import QtQuick
import QtQuick.Controls

Button {
    id: control
    property bool primary: false
    implicitHeight: 38
    implicitWidth: Math.max(70, contentItem.implicitWidth + 28)
    hoverEnabled: true
    font.pixelSize: 13
    font.weight: Font.Medium
    contentItem: Text {
        text: control.text
        font: control.font
        color: control.primary ? theme.colors.onAccent : theme.colors.text
        opacity: control.enabled ? 1 : 0.4
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
    background: Rectangle {
        radius: 5
        color: control.primary ? theme.colors.accent : control.down ? theme.colors.line : control.hovered ? theme.colors.surface : "transparent"
        border.width: control.visualFocus ? 2 : 1
        border.color: control.visualFocus ? theme.colors.accent : control.primary ? theme.colors.accent : theme.colors.line
        opacity: control.enabled ? 1 : 0.5
    }
}
