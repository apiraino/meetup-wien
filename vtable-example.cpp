#include <iostream>
using namespace std;

class Draw
{
    public:
      virtual void draw();
};

class Button : public Draw
{
    public:
       void draw()
         { this->Draw::draw();
           cout << "Drawing button.\n"; 
         }
};

int main()
{
    Draw *w;
    Button b;

    w = &b;
    w->draw();

    return 0;
}