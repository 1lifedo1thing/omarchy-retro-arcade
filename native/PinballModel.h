#pragma once
#include <array>
#include <vector>
#include <string>
#include <iosfwd>
namespace oma {
struct Vec { double x=0,y=0; Vec()=default; Vec(double a,double b):x(a),y(b){} Vec operator+(Vec v)const{return {x+v.x,y+v.y};} Vec operator-(Vec v)const{return {x-v.x,y-v.y};} Vec operator*(double k)const{return {x*k,y*k};} };
struct Segment { Vec a,b; };
struct Input { bool left=false,right=false,plunger=false; };
enum class Event { Bumper, Target, Flipper, Launch, Drain, Mission, Tilt, Save };
struct Model {
    Vec ball{555,824},velocity{};
    bool waiting=true,gameOver=false,paused=false,tilted=false;
    int balls=3,score=0,multiplier=1,circuits=0;
    double leftAngle=.35,rightAngle=2.791592653589793,charge=0,time=0,saveTime=0,nudgeHeat=0;
    std::array<bool,3> lanes{{false,false,false}},targets{{false,false,false}};
    std::array<double,3> bumperGlow{{0,0,0}},targetGlow{{0,0,0}};
    std::vector<Event> events;
    std::string message="Hold SPACE to charge. Release to launch.";
    void step(double dt,Input input);
    void nudge();
    void newGame();
    void launch(double power);
    bool write(std::ostream&)const;
    bool read(std::istream&);
    static const std::vector<Segment>& walls();
    static std::array<Vec,3> bumpers();
    static std::array<Vec,3> lanePositions();
    static std::array<Vec,3> targetPositions();
    Vec leftTip()const; Vec rightTip()const;
private:
    bool wasPlunger=false;
    void collideSegment(Vec a,Vec b,double restitution,Vec surface={});
    void collideCircle(Vec center,double radius,double restitution,double boost);
    void addScore(int points);
    void drain();
};
}
