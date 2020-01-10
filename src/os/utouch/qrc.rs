qrc!(qml_resources,
    "/" {
        "src/utouch/qml/Main.qml"
    },
);

pub fn load() {
    qml_resources();
}
