//  g++ -O2 -o bench bench.cpp

#include <iostream>
using namespace std;

class Draw
{
public:
    virtual void draw(){};
};

class Button : public Draw
{
public:
    void draw()
    {
        // this->Draw::draw();
        cout << "Drawing button.\n";
    }
};

int main()
{
    // Draw *w;
    Draw *b = new Button();

    // w = &b;
    for (int i=0; i<1000000; i++) {
        b->draw();
    }

    return 0;
}
