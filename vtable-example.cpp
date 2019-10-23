#include <iostream>
using namespace std;

class Weapon
{
    public:
      virtual void features();
};

class Bomb : public Weapon
{
    public:
       void features()
         { this->Weapon::features();
           cout << "Loading bomb features.\n"; 
         }
};

class Loader
{
   public:
     void loadFeatures(Weapon *weapon)
     {
        weapon->features();
     }     
};

int main()
{
    Loader *l = new Loader;
    Weapon *w;
    Bomb b;

    w = &b;
    w->features();

    return 0;
}