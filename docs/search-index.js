var searchIndex = JSON.parse('{\
"rat_rs":{"doc":"rat-rs","t":[17,0,0,5,5,0,0,13,13,13,3,13,13,13,13,13,4,4,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,3,3,3,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,3,12,11,11,11,11,11,5,11,11,11],"n":["API_URI","args","error","main","run_main","structs","utils","A","AR","Bus","CliArgs","Metro","Noctilien","R","Rer","Tramway","TransportType","WayType","augment_args","augment_args_for_update","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","code","default","fmt","fmt","fmt","fmt","fmt","from","from","from","from_arg_matches","from_arg_matches_mut","into","into","into","into_app","into_app_for_update","station","to_owned","to_owned","to_possible_value","to_possible_value","to_string","to_string","transport_type","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","update_from_arg_matches","update_from_arg_matches_mut","value_variants","value_variants","way","CliError","RequestError","borrow","borrow_mut","clone","clone_into","error_code","fmt","fmt","from","from","into","report","to_owned","to_string","try_from","try_into","type_id","0","Response","ResponseResult","Schedule","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","deserialize","deserialize","deserialize","destination","fmt","fmt","fmt","fmt","from","from","from","into","into","into","message","result","schedules","serialize","serialize","serialize","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","REPLACEABLE","__private_field","borrow","borrow_mut","deref","from","into","slugify","try_from","try_into","type_id"],"q":["rat_rs","","","","","","","rat_rs::args","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","rat_rs::error","","","","","","","","","","","","","","","","","","rat_rs::error::CliError","rat_rs::structs","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","rat_rs::utils","","","","","","","","","",""],"d":["Our API endpoint","Application arguments","Application errors","","","Structures used by the CLI","Application tools","","A+R means any direction","","","","","","","","A transport type is a way to go from a point A to a point B","A way type is similar to a direction","","","","","","","","","","","","","Code of the transport","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Station where you would like to have the next schedules","","","","","","","Desired transport type","","","","","","","","","","","","","","What direction you want to go","Common errors that can be thrown on runtime","An error sent on request to an external library","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","The overall response, contains our response result and …","The result inside the response from the distant endpoint","The fetched schedules","","","","","","","","","","The destination","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","A message contains either a time (ie 11mn) or a temporal …","The result of our request","The list of schedules that we will use to display our …","","","","","","","","","","","","","","The regex used to slugify our inputs","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","This method transforms a string into a slug to request the …","","",""],"i":[0,0,0,0,0,0,0,1,1,2,0,2,2,1,2,2,0,0,3,3,3,2,1,3,2,1,2,1,2,1,3,1,3,2,2,1,1,3,2,1,3,3,3,2,1,3,3,3,2,1,2,1,2,1,3,3,2,1,3,2,1,3,2,1,3,3,2,1,3,0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,0,0,0,6,7,8,6,7,8,6,7,8,8,6,7,8,8,6,7,8,6,7,8,8,7,6,6,7,8,8,6,7,8,6,7,8,6,7,8,0,9,9,9,9,9,9,0,9,9,9],"f":[null,null,null,[[],["exitcode",3]],[[]],null,null,null,null,null,null,null,null,null,null,null,null,null,[[["command",6]],["command",6]],[[["command",6]],["command",6]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["transporttype",4]],[[["",0]],["waytype",4]],[[["",0],["",0]]],[[["",0],["",0]]],null,[[],["waytype",4]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[["argmatches",3]],["result",4,[["error",3]]]],[[["argmatches",3]],["result",4,[["error",3]]]],[[]],[[]],[[]],[[],["command",6]],[[],["command",6]],null,[[["",0]]],[[["",0]]],[[["",0]],["option",4,[["possiblevalue",3]]]],[[["",0]],["option",4,[["possiblevalue",3]]]],[[["",0]],["string",3]],[[["",0]],["string",3]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0],["argmatches",3]],["result",4,[["error",3]]]],[[["",0],["argmatches",3]],["result",4,[["error",3]]]],[[]],[[]],null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["clierror",4]],[[["",0],["",0]]],[[["",0]],["u8",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["error",3]]],[[]],[[]],[[],["exitcode",3]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[],["result",4]],[[],["result",4]],[[],["result",4]],null,[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["regex",3]],[[]],[[]],[[["str",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]]],"p":[[4,"WayType"],[4,"TransportType"],[3,"CliArgs"],[4,"CliError"],[13,"RequestError"],[3,"ResponseResult"],[3,"Response"],[3,"Schedule"],[3,"REPLACEABLE"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};