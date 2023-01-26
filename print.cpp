#include<iostream>
using namespace std;

extern "C"{

    void println(){
        cout<<"hello cpp"<<endl;
    }
}