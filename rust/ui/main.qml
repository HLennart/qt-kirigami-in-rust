import QtQuick 2.0
import QtQuick.Controls 2.12 as Controls
import QtQuick.Window 2.2
import org.kde.kirigami 2.13 as Kirigami

Kirigami.ApplicationWindow {
    id: root
    visible: true
    width: 320; height: 480
    color: "lightgray"

    pageStack.initialPage: Kirigami.Page {
        Controls.Label {
            anchors.centerIn: parent
            text: "Hello World!"
        }
    }

    // Text {
    //     id: helloText
    //     text: "Hello World"
    //     y: 30
    //     // anchors.horizontalCenter: page.horizontalCenter
    //     font.pointSize: 24; font.bold: true
    // }
}