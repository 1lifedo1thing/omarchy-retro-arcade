#pragma once
#include <vector>
#include <cmath>

// Measured in the unchanged 1536 x 1024 plate. This is the authored table,
// shared by the upstream DAT builder and the foreground compositor.
namespace CircuitLayout {
struct Point { float x,y; };
struct Body { const char* name; std::vector<Point> outline; };
struct Round { float x,y,r; };
inline float worldX(float x){return (x-540)/25;}
inline float worldY(float y){return (y-500)/25;}
inline float imageX(float x){return 540+x*25;}
inline float imageY(float y){return 500+y*25;}
inline const std::vector<Body>& bodies(){
 static const std::vector<Body> v={
  {"sling_left_body",{{295,635},{269,735},{360,778}}},
  {"sling_right_body",{{755,635},{690,778},{788,735}}},
  {"target_body0",{{506,95},{526,95},{526,145},{506,145}}},
  {"target_body1",{{535,95},{555,95},{555,145},{535,145}}},
  {"target_body2",{{564,95},{584,95},{584,145},{564,145}}},
  {"target_body3",{{593,95},{613,95},{613,145},{593,145}}},
  {"module_body",{{744,319},{779,328},{750,438},{714,426}}},
  {"upper_bank",{{641,116},{676,134},{719,179},{750,232},{745,259},{730,258},{709,207},{665,166},{641,151}}},
  {"left_return",{{208,594},{214,730},{358,837},{352,849},{205,744},{199,726}}},
  {"right_return",{{834,751},{826,766},{711,840},{716,849},{838,776},{846,758}}},
  {"left_apron",{{188,800},{302,895},{291,954},{458,1024},{476,1024},{307,945},{315,889},{199,790}}},
  {"right_apron",{{830,801},{754,893},{765,953},{610,1024},{594,1024},{751,943},{742,889},{818,795}}},
  // Tube separating the left feature lane from the open centre of the table.
  {"left_guide",{{305,579},{296,491},{281,465},{291,419},{303,384},{309,329},{318,331},{312,387},{300,422},{291,463},{306,488},{314,579}}},
  {"left_feature",{{194,550},{182,537},{191,493},{212,464},{258,432},{278,397},{286,367},{295,371},{287,405},{268,441},{219,473},{202,500},{194,535}}},
  {"shooter_guide",{{890,870},{858,488},{850,417},{850,395},{858,395},{860,421},{868,485},{900,870}}}
 };
 return v;
}
inline const std::vector<Round>& posts(){
 static const std::vector<Round> v={{434,300,11},{413,388,10},{308,579,10},
 {775,469,15},{842,490,12},{768,618,8},{295,635,8},{269,735,8},{360,778,8},
 {755,635,8},{788,735,8},{690,778,8},{208,604,8},{199,744,8},
 {318,831,8},{744,829,8},{291,955,8},{766,955,8}};
 return v;
}
inline const std::vector<Round>& bumpers(){
 static const std::vector<Round> v={{470,232,39},{617,207,39},{563,290,39},{202,433,29}};
 return v;
}
inline const std::vector<Point>& rampCentres(){
 static const std::vector<Point> v={{374,400},{373,350},{365,310},{345,280},{298,261},
 {258,240},{242,209},{244,165},{265,119},{307,79},{352,57},{385,57},{395,74},{392,88},{414,99}};
 return v;
}
inline float rampHalfWidth(unsigned i){return i+2>=rampCentres().size()?15.f:24.f;}
inline void rampEdges(std::vector<Point>& left,std::vector<Point>& right){
 const auto& c=rampCentres();
 for(unsigned i=0;i<c.size();++i){auto a=c[i?i-1:i],b=c[i+1<c.size()?i+1:i];
  float dx=b.x-a.x,dy=b.y-a.y,n=std::hypot(dx,dy),w=rampHalfWidth(i);
  left.push_back({c[i].x-dy/n*w,c[i].y+dx/n*w});
  right.push_back({c[i].x+dy/n*w,c[i].y-dx/n*w});
 }
}
// The visible launch head at the bottom of the shooter lane.
constexpr float plungerLeft=919,plungerRight=972,plungerY=935,feedX=942,feedY=906;
}
