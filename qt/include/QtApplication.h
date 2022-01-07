#pragma once

#include <QApplication>
#include <memory>

namespace qt {
class QtApplication {
public:
    QtApplication();
    void run() const;
private:
    QApplication _app;
};

std::unique_ptr<QtApplication> new_qapplication();
}



