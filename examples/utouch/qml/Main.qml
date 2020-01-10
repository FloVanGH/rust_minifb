import QtQuick 2.7
import QtQuick.Controls 2.2
import Ubuntu.Components 1.3
import QtQuick.Layouts 1.3
import Qt.labs.settings 1.0

import Greeter 1.0

ApplicationWindow {
    id: root
    objectName: 'mainView'

    width: units.gu(45)
    height: units.gu(75)
    visible: true

    Greeter {
        id: greeter
        name: "Rust + Ubuntu Touch"
    }

    Page {
        anchors.fill: parent

        header: PageHeader {
            id: header
            title: i18n.tr('utouch')
        }

        ColumnLayout {
            spacing: units.gu(2)
            anchors {
                margins: units.gu(2)
                top: header.bottom
                left: parent.left
                right: parent.right
                bottom: parent.bottom
            }

            Item {
                Layout.fillHeight: true
            }

            Label {
                id: label
                text: i18n.tr('Press the button below!')
            }

            Button {
                text: i18n.tr('Compute greeting')
                onClicked: {
                    label.text = greeter.compute_greetings("Hello, ");
                }
            }

            Item {
                Layout.fillHeight: true
            }
        }
    }
}
