qrc!(qml_resources,
    "/" {
        "src/os/utouch/qml/Main.qml"
    },
);

pub fn load() {
    qml_resources();
}
