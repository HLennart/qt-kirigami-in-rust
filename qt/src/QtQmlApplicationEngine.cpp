#include "qt/include/QtQmlApplicationEngine.h"
#include <QUrl>
#include <QString>

namespace qt {

QtQmlApplicationEngine::QtQmlApplicationEngine(std::string qmlPath): _engine(QUrl(QString::fromStdString(qmlPath))) {}

// QtQmlApplicationEngine::QtQmlApplicationEngine(std::string qmlPath): _engine(QUrl(qmlPath)) {}

std::unique_ptr<QtQmlApplicationEngine> new_qt_qml_application_engine(rust::Str path) {
    return std::unique_ptr<QtQmlApplicationEngine> (new QtQmlApplicationEngine(std::string(path)));
}

}