#pragma once
#include <QQmlApplicationEngine>
#include <string>
#include "rust/cxx.h"

namespace qt {
class QtQmlApplicationEngine {
public:
    QtQmlApplicationEngine(std::string qmlPath);
private: 
    QQmlApplicationEngine _engine;
};

std::unique_ptr<QtQmlApplicationEngine> new_qt_qml_application_engine(rust::Str path);
}