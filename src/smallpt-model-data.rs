

 //-------------Debug Scene sc1-------------

 lazy_static! {
    static ref SPHERES: [Sphere; 9] = [
        Sphere { rad: 1e5,   p: Vec3::new( 1e5 + 1.0,      40.8, 81.6),e: Vec3::zero(),               c: Vec3::new(0.75, 0.25, 0.25), refl: Refl::Diff },//left
        Sphere { rad: 1e5,   p: Vec3::new(-1e5 + 99.0,    40.8, 81.6),e: Vec3::zero(),                c: Vec3::new(0.25, 0.25, 0.75), refl: Refl::Diff },//right
        Sphere { rad: 1e5,   p: Vec3::new(50.0,            40.8, 1e5),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },//front
        Sphere { rad: 1e5,   p: Vec3::new(50.0,    40.8,-1e5 + 170.0),e: Vec3::zero(),                c: Vec3::zero(), refl: Refl::Diff },//back
        Sphere { rad: 1e5,   p: Vec3::new(50.0,            1e5, 81.6),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },//bottom
        Sphere { rad: 1e5,   p: Vec3::new(50.0,-1e5 + 81.6+4.0, 81.6),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },//top
        Sphere { rad: 16.5,  p: Vec3::new(27.0,           16.5, 47.0),e: Vec3::zero(),                c: Vec3::new(1.0, 1.0, 1.0) * 0.999, refl: Refl::Spec },
        Sphere { rad: 16.5,  p: Vec3::new(73.0,           16.5, 78.0),e: Vec3::zero(),                c: Vec3::new(1.0, 1.0, 1.0) * 0.999, refl: Refl::Refr },
        Sphere { rad: 600.0, p: Vec3::new(50.0, 681.6-0.27+4.0, 81.6),e: Vec3::new(12.0, 12.0, 12.0), c: Vec3::zero(), refl: Refl::Diff },
	];
}

   //----------cornel box sc1-----------
lazy_static! {
    static ref SPHERES: [Sphere; 9] = [
        Sphere { rad: 1e5,  p: Vec3::new(1e5 + 1.0,      40.8, 81.6),e: Vec3::zero(),                c: Vec3::new(0.75, 0.25, 0.25), refl: Refl::Diff },
        Sphere { rad: 1e5,  p: Vec3::new(-1e5 + 99.0,    40.8, 81.6),e: Vec3::zero(),                c: Vec3::new(0.25, 0.25, 0.75), refl: Refl::Diff },
        Sphere { rad: 1e5,  p: Vec3::new(50.0,           40.8, 1e5 ),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },
        Sphere { rad: 1e5,  p: Vec3::new(50.0,    40.8,-1e5 + 170.0),e: Vec3::zero(),         c: Vec3::zero(), refl: Refl::Diff },
        Sphere { rad: 1e5,  p: Vec3::new(50.0,            1e5, 81.6),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },
        Sphere { rad: 1e5,  p: Vec3::new(50.0,-1e5 + 81.6+4.0, 81.6),e: Vec3::zero(),                c: Vec3::new(0.75, 0.75, 0.75), refl: Refl::Diff },
        Sphere { rad: 16.5, p: Vec3::new(27.0,           16.5, 47.0),e: Vec3::zero(),                c: Vec3::new(1.0, 1.0, 1.0) * 0.999, refl: Refl::Spec },
        Sphere { rad: 16.5, p: Vec3::new(73.0,           16.5, 78.0),e: Vec3::zero(),                c: Vec3::new(1.0, 1.0, 1.0) * 0.999, refl: Refl::Refr },
        Sphere { rad: 1.5,  p: Vec3::new(50.0,      81.6-16.5, 81.6),e: Vec3::new(4.0,4.0,4.0)*100.0,c: Vec3::zero(), refl: Refl::Diff },
    ];
}

  //-----------sky sc2--------------
const Cen:Vec3=Vec3{x:50.0,y:40.8,z:-860.0};
lazy_static! {
    static ref SPHERES: [Sphere; 9] = [
		Sphere { rad:1600.0, p:Vec3::new(1.0,0.0,2.0)*3000.0, e:Vec3::new(1.0,0.9,0.8)*1.2e1*1.56*2.0,  c:Vec3::zero(), refl: Refl::Diff}, // sun
		Sphere { rad:1560.0, p:Vec3::new(1.0,0.0,2.0)*3500.0, e:Vec3::new(1.0,0.5,0.05)*4.8e1*1.56*2.0, c:Vec3::zero(), refl: Refl::Diff}, // horizon sun2
        Sphere { rad:10000.0,p:Cen+Vec3::new(0.0,0.0,-200.0), e:Vec3::new(0.00063842, 0.02001478, 0.28923243)*6e-2*8.0, c:Vec3::new(0.7,0.7,1.0)*0.25,  refl: Refl::Diff}, // sky

		Sphere { rad:100000.0,p:Vec3::new(50.0,-100000.0,0.0),e:Vec3::zero(),				c:Vec3::new(0.3,0.3,0.3),refl: Refl::Diff}, // grnd
		Sphere { rad:110000.0,p:Vec3::new(50.0,-110048.5,0.0),e:Vec3::new(0.9,0.5,0.05)*4.0,c:Vec3::zero(),refl: Refl::Diff},// horizon brightener
		Sphere { rad:4e4, 	  p:Vec3::new(50.0,-4e4-30.0,-3000.0),e:Vec3::zero(),			c:Vec3::new(0.2,0.2,0.2),refl: Refl::Diff},// mountains

        Sphere { rad:26.5,	p:Vec3::new(22.0,26.5,42.0),e:Vec3::zero(),						c:Vec3::new(1.0,1.0,1.0)*0.596, refl: Refl::Spec}, // white Mirr
        Sphere { rad:13.0,	p:Vec3::new(75.0,13.0,82.0),e:Vec3::zero(),						c:Vec3::new(0.96,0.96,0.96)*0.96, refl: Refl::Refr},// Glas
        Sphere { rad:22.0,	p:Vec3::new(87.0,22.0,24.0),e:Vec3::zero(),						c:Vec3::new(0.6,0.6,0.6)*0.696, refl: Refl::Refr},    // Glas2
	];
}


  //------------nightsky sc3----

lazy_static! {
    static ref SPHERES: [Sphere; 12] = [
  Sphere { rad:2.5e3,p:Vec3::new(0.82,0.92,-2.0)*1e4,   e:Vec3::new(1.0,1.0,1.0)*0.8e2,        c:Vec3::zero(), refl: Refl::Diff}, // moon
  Sphere { rad:2.5e4,p:Vec3::new(50.0, 0.0, 0.0),       e:Vec3::new(0.114, 0.133, 0.212)*1e-2, c:Vec3::new(0.216,0.384,1.0)*0.003, refl: Refl::Diff}, // sky
  Sphere { rad:5e0,  p:Vec3::new(-0.2,0.16,-1.0)*1e4,   e:Vec3::new(1.00, 0.843, 0.698)*1e2,   c:Vec3::zero(), refl: Refl::Diff},  // star
  Sphere { rad:5e0,  p:Vec3::new(0.0,0.18,-1.0)*1e4,    e:Vec3::new(1.00, 0.851, 0.710)*1e2,   c:Vec3::zero(), refl: Refl::Diff},  // star
  Sphere { rad:5e0,  p:Vec3::new(0.3,0.15,-1.0)*1e4,    e:Vec3::new(0.671, 0.780, 1.00)*1e2,   c:Vec3::zero(), refl: Refl::Diff},  // star
  Sphere { rad:3.5e4,p:Vec3::new(600.0,-3.5e4+1.0,300.0), e:Vec3::zero(),                      c:Vec3::new(0.6,0.8,1.0)*0.01,  refl: Refl::Refr},   //pool
  Sphere { rad:5e4,  p:Vec3::new(-500.0,-5e4  ,0.0),    e:Vec3::zero(),                        c:Vec3::new(1.0,1.0,1.0)*0.35,  refl: Refl::Diff},    //hill
  Sphere { rad:16.5, p:Vec3::new(27.0,0.0,47.0),        e:Vec3::zero(),                        c:Vec3::new(1.0,1.0,1.0)*0.33, refl: Refl::Diff}, //hut
  Sphere { rad:7.0,  p:Vec3::new(27.0+8.0*SQRT_2,0.0,47.0+8.0*SQRT_2),e:Vec3::zero(),          c:Vec3::new(1.0,1.0,1.0)*0.33,  refl: Refl::Diff}, //door
  Sphere { rad:500.0,p:Vec3::new(-1e3,-300.0,-3e3),     e:Vec3::zero(),                        c:Vec3::new(1.0,1.0,1.0)*0.351,    refl: Refl::Diff},  //mnt
  Sphere { rad:830.0,p:Vec3::new(0.0,-500.0,-3e3),     e:Vec3::zero(),                         c:Vec3::new(1.0,1.0,1.0)*0.354,    refl: Refl::Diff},  //mnt
  Sphere { rad:490.0,p:Vec3::new(1e3,-300.0,-3e3),      e:Vec3::zero(),                        c:Vec3::new(1.0,1.0,1.0)*0.352,    refl: Refl::Diff},  //mnt
	];
}

//-----------island sc4-------

const  Cen:Vec3=Vec3{x:50.0,y:-20.0,z:-860.0};

lazy_static! {
	static ref SPHERES: [Sphere;7] = [
		
  Sphere { rad:160.0, p:Cen+Vec3::new(0.0, 600.0, -500.0), e:Vec3::new(1.0,1.0,1.0)*2e2,          c:Vec3::zero(),  refl: Refl::Diff}, // sun
  Sphere { rad:800.0, p:Cen+Vec3::new(0.0,-880.0,-9120.0), e:Vec3::new(1.0,1.0,1.0)*2e1,          c:Vec3::zero(),  refl: Refl::Diff}, // horizon
  Sphere { rad:10000.0,p:Cen+Vec3::new(0.0,0.0,-200.00),   e:Vec3::new(0.0627, 0.188, 0.569)*1e0, c:Vec3::new(1.0,1.0,1.0)*0.4,  refl: Refl::Diff}, // sky
  Sphere { rad:800.0, p:Cen+Vec3::new(0.0,-720.0,-200.0),  e:Vec3::zero(),                        c:Vec3::new(0.110, 0.898, 1.00)*0.996,  refl: Refl::Refr}, // water
  Sphere { rad:790.0, p:Cen+Vec3::new(0.0,-720.0,-200.0),  e:Vec3::zero(),                        c:Vec3::new(0.4,0.3,0.04)*0.6, refl: Refl::Diff}, // earth
  Sphere { rad:325.0, p:Cen+Vec3::new(0.0,-255.0,-50.0),   e:Vec3::zero(),                        c:Vec3::new(0.4,0.3,0.04)*0.8, refl: Refl::Diff}, // island
  Sphere { rad:275.0, p:Cen+Vec3::new(0.0,-205.0,-33.0),   e:Vec3::zero(),                        c:Vec3::new(0.02,0.3,0.02)*0.75,refl: Refl::Diff}, // grass
	];
}

  //-------------Vista sc5------------

const  Cen:Vec3=Vec3{x:50.0,y:-20.0,z:-860.0};


lazy_static! {
	static ref SPHERES: [Sphere;24] = [


  Sphere { rad:8000.0, p:Cen+Vec3::new(0.0,-8000.0,-900.0),e:Vec3::new(1.0,0.4,0.1)*5e-1,        c:Vec3::zero(),  refl: Refl::Diff}, // sun
  Sphere { rad:1e4,    p:Cen+Vec3::zero(),                 e:Vec3::new(0.631, 0.753, 1.00)*3e-1, c:Vec3::new(1.0,1.0,1.0)*0.5,  refl: Refl::Diff}, // sky

  Sphere { rad:150.0,  p:Cen+Vec3::new(-350.0, 0.0,-100.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.3,  refl: Refl::Diff}, // mnt
  Sphere { rad:200.0,  p:Cen+Vec3::new(-210.0, 0.0,-100.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.3,  refl: Refl::Diff}, // mnt
  Sphere { rad:145.0,  p:Cen+Vec3::new(-210.0,85.0,-100.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff}, // snow
  Sphere { rad:150.0,  p:Cen+Vec3::new(-50.0,  0.0,-100.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.3,  refl: Refl::Diff}, // mnt
  Sphere { rad:150.0,  p:Cen+Vec3::new(100.0,  0.0,-100.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.3,  refl: Refl::Diff}, // mnt
  Sphere { rad:125.0,  p:Cen+Vec3::new(250.0,  0.0,-100.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.3,  refl: Refl::Diff}, // mnt
  Sphere { rad:150.0,  p:Cen+Vec3::new(375.0,  0.0,-100.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.3,  refl: Refl::Diff}, // mnt

  Sphere { rad:2500.0, p:Cen+Vec3::new(0.0,-2400.0,-500.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.1,  refl: Refl::Diff}, // mnt base

  Sphere { rad:8000.0, p:Cen+Vec3::new(0.0,-8000.0, 200.0), e:Vec3::zero(),                       c:Vec3::new(0.2,0.2,1.0),    refl: Refl::Refr}, // water
  Sphere { rad:8000.0, p:Cen+Vec3::new(0.0,-8000.0,1100.0), e:Vec3::zero(),                       c:Vec3::new(0.0,0.3,0.0),   refl: Refl::Diff}, // grass
  Sphere { rad:8.0   , p:Cen+Vec3::new(-75.0, -5.0, 850.0), e:Vec3::zero(),                       c:Vec3::new(0.0,0.3,0.0),     refl: Refl::Diff}, // bush
  Sphere { rad:30.0,   p:Cen+Vec3::new(0.0,   23.0, 825.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.996, refl: Refl::Refr}, // ball

  Sphere { rad:30.0,   p:Cen+Vec3::new(200.0,280.0,-400.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},   // clouds
  Sphere { rad:37.0,   p:Cen+Vec3::new(237.0,280.0,-400.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},   // clouds
  Sphere { rad:28.0,   p:Cen+Vec3::new(267.0,280.0,-400.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},   // clouds

  Sphere { rad:40.0,   p:Cen+Vec3::new(150.0,280.0,-1000.0),e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},  // clouds
  Sphere { rad:37.0,   p:Cen+Vec3::new(187.0,280.0,-1000.0),e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},  // clouds

  Sphere { rad:40.0,   p:Cen+Vec3::new(600.0,280.0,-1100.0),e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},  // clouds
  Sphere { rad:37.0,   p:Cen+Vec3::new(637.0,280.0,-1100.0),e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},  // clouds

  Sphere { rad:37.0, p:Cen+Vec3::new(-800.0,280.0,-1400.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff}, // clouds
  Sphere { rad:37.0, p:Cen+Vec3::new(0.0,   280.0,-1600.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},    // clouds
  Sphere { rad:37.0, p:Cen+Vec3::new(537.0, 280.0,-1800.0), e:Vec3::zero(),                       c:Vec3::new(1.0,1.0,1.0)*0.8,  refl: Refl::Diff},  // clouds
	];
}



//----------------Overlap  sc6-----------------

const D:f64=50.0;
const R:f64=40.0;

lazy_static! {
	static ref SPHERES: [Sphere;3] = [

  Sphere { rad:150.0, p:Vec3::new(50.0+75.0,28.0,62.0), e:Vec3::new(1.0,1.0,1.0)*0e-3, c:Vec3::new(1.0,0.9,0.8)*0.93, refl: Refl::Refr},
  Sphere { rad:28.0,  p:Vec3::new(50.0+5.0,-28.0,62.0), e:Vec3::new(1.0,1.0,1.0)*1e1,  c:Vec3::zero(), refl: Refl::Diff},
  Sphere { rad:300.0, p:Vec3::new(50.0,     28.0,62.0), e:Vec3::new(1.0,1.0,1.0)*0e-3, c:Vec3::new(1.0,1.0,1.0)*0.93, refl: Refl::Spec},
	];
}


  //----------------wada  sc7-------------

R:=60;
//double R=120;
T:=30*PI/180.;
D:=R/cos(T},
Z:=60;

  Sphere { rad:1e5, Vec3::new(50, 100, 0.0),      Vec3::new(1.0,1.0,1.0)*3e0, Vec3::zero(), refl: Refl::Diff}, // sky
  Sphere { rad:1e5, Vec3::new(50, -1e5-D-R, 0.0), Vec3::zero(),     Vec3::new(0.1,0.1,0.1),refl: Refl::Diff},           //grnd

  Sphere { rad:R, Vec3::new(50,40.8,62)+Vec3::new( cos(T),sin(T),0)*D, Vec3::zero(), Vec3::new(1,0.3,0.3)*0.999, refl: Refl::Spec)}, //red
  Sphere { rad:R, Vec3::new(50,40.8,62)+Vec3::new(-cos(T),sin(T),0)*D, Vec3::zero(), Vec3::new(0.3,1,0.3)*0.999, refl: Refl::Spec)}, //grn
  Sphere { rad:R, Vec3::new(50,40.8,62)+Vec3::new(0,-1,0)*D,         Vec3::zero(), Vec3::new(0.3,0.3,1)*0.999, refl: Refl::Spec)}, //blue
  Sphere { rad:R, Vec3::new(50,40.8,62)+Vec3::new(0,0,-1)*D,       Vec3::zero(), Vec3::new(0.53,0.53,0.53)*0.999, refl: Refl::Spec)}, //back
  Sphere { rad:R, Vec3::new(50,40.8,62)+Vec3::new(0,0,1)*D,      Vec3::zero(), Vec3::new(1.0,1.0,1.0)*0.999, refl: Refl::Refr)}, //front


  //-----------------wada2 sc8----------

R:=120;     // radius
T:=30*PI/180.;
D:=R/cos(T},     //distance
Z:=62;
C:=Vec3::new(0.275, 0.612, 0.949},

  Sphere { rad:R, Vec3::new(50,28,Z)+Vec3::new( cos(T),sin(T),0)*D,    C*6e-2,Vec3::new(1.0,1.0,1.0)*0.996, refl: Refl::Spec)}, //red
  Sphere { rad:R, Vec3::new(50,28,Z)+Vec3::new(-cos(T),sin(T),0)*D,    C*6e-2,Vec3::new(1.0,1.0,1.0)*0.996, refl: Refl::Spec)}, //grn
  Sphere { rad:R, Vec3::new(50,28,Z)+Vec3::new(0,-1,0)*D,              C*6e-2,Vec3::new(1.0,1.0,1.0)*0.996, refl: Refl::Spec)}, //blue
  Sphere { rad:R, Vec3::new(50,28,Z)+Vec3::new(0,0,-1)*R*2*sqrt(2/3),C*0e-2,Vec3::new(1.0,1.0,1.0)*0.996, refl: Refl::Spec)}, //back
  Sphere { rad:2*2*R*2*sqrt(2/3)-R*2*sqrt(2/3)/3, Vec3::new(50,28,Z)+Vec3::new(0,0,-R*2*sqrt(2/3)/3), Vec3::new(1.0,1.0,1.0)*0,Vec3::new(1.0,1.0,1.0)*0.5, refl: Refl::Spec)}, //front



  //---------------forest sc9-----------

tc:=Vec3::new(0.0588, 0.361, 0.0941},
scc:=Vec3::new(1.0,1.0,1.0)*0.7;
  Sphere { rad:1e5, Vec3::new(50, 1e5+130, 0.0),  Vec3::new(1.0,1.0,1.0)*1.3,Vec3::zero(),refl: Refl::Diff}, //lite
  Sphere { rad:1e2, Vec3::new(50, -1e2+2, 47),  Vec3::zero(),Vec3::new(1.0,1.0,1.0)*0.7,refl: Refl::Diff}, //grnd

  Sphere { rad:1e4, Vec3::new(50, -30, 300)+Vec3::new(-sin(50*PI/180.0),0,cos(50*PI/180))*1e4, Vec3::zero(), Vec3::new(1.0,1.0,1.0)*0.99,refl: Refl::Spec)},// mirr L
  Sphere { rad:1e4, Vec3::new(50, -30, 300)+Vec3::new(sin(50*PI/180.0),0,cos(50*PI/180))*1e4,  Vec3::zero(), Vec3::new(1.0,1.0,1.0)*0.99,refl: Refl::Spec)},// mirr R
  Sphere { rad:1e4, Vec3::new(50, -30, -50)+Vec3::new(-sin(30*PI/180.0),0,-cos(30*PI/180))*1e4,Vec3::zero(), Vec3::new(1.0,1.0,1.0)*0.99,refl: Refl::Spec)},// mirr FL
  Sphere { rad:1e4, Vec3::new(50, -30, -50)+Vec3::new(sin(30*PI/180.0),0,-cos(30*PI/180))*1e4, Vec3::zero(), Vec3::new(1.0,1.0,1.0)*0.99,refl: Refl::Spec)},// mirr


  Sphere { rad:4, Vec3::new(50,6*0.6,47),   Vec3::zero(),Vec3::new(0.13,0.066,0.033), refl: Refl::Diff},//"tree"
  Sphere { rad:16,Vec3::new(50,6*2+16*0.6,47),   Vec3::zero(), tc,  refl: Refl::Diff},//"tree"
  Sphere { rad:11,Vec3::new(50,6*2+16*0.6*2+11*0.6,47),   Vec3::zero(), tc,  refl: Refl::Diff},//"tree"
  Sphere { rad:7, Vec3::new(50,6*2+16*0.6*2+11*0.6*2+7*0.6,47),   Vec3::zero(), tc,  refl: Refl::Diff},//"tree"

  Sphere { rad:15.5,Vec3::new(50,1.8+6*2+16*0.6,47),   Vec3::zero(), scc,  refl: Refl::Diff},//"tree"
  Sphere { rad:10.5,Vec3::new(50,1.8+6*2+16*0.6*2+11*0.6,47),   Vec3::zero(), scc,  refl: Refl::Diff},//"tree"
  Sphere { rad:6.5, Vec3::new(50,1.8+6*2+16*0.6*2+11*0.6*2+7*0.6,47),   Vec3::zero(), scc,  refl: Refl::Diff},//"tree"

