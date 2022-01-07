#include "qt/include/QtApplication.h"
#include <QPushButton>

namespace qt {
static int NAUGHT = 1;

QtApplication::QtApplication(): _app(NAUGHT, nullptr) {}

void QtApplication::run() const {
    // QPushButton button("Hello World");
    // button.show();
    
    _app.exec();
};

std::unique_ptr<QtApplication> new_qapplication() {
    return std::unique_ptr<QtApplication>(new QtApplication());
}
}
