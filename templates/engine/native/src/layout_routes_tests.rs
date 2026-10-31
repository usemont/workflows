use super::*;
#[test]fn source_ports_keep_ten_pixel_clearance(){let n=json!({"x":100,"y":200,"w":80,"h":60});assert_eq!(port(&n,"r",false).p,(190.0,230.0));assert_eq!(port(&n,"t",true).p,(140.0,190.0));assert_eq!(port(&n,"t",true).d,(0.0,1.0));}
#[test]fn angled_source_rounds_each_bend_with_eight_samples(){let a=Port{p:(0.0,0.0),d:(1.0,0.0)};let b=Port{p:(100.0,100.0),d:(1.0,0.0)};let p=angled(a,b,22.0);assert_eq!(p.len(),20);assert_eq!(p[0],a.p);assert_eq!(*p.last().unwrap(),b.p);assert_eq!(p[1],(28.0,0.0));}
#[test]fn source_curves_have_sixty_four_segments(){let a=Port{p:(0.0,0.0),d:(1.0,0.0)};let b=Port{p:(100.0,100.0),d:(0.0,1.0)};let p=curve(a,b);assert_eq!(p.len(),65);assert_eq!(p[0],a.p);assert_eq!(p[64],b.p);}
