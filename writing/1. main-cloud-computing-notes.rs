// it's setting there lessoning for web browsers to make connects. lessons for incoming connections. And give information. Imagen 1 people ask for data.
// why some of this service so big? they are normally try to handle many more cases and they're trying to do things much faster.
//Imagen 2 people ask for data at vertually the same time, one by one will handle them. whoever gets there first will handle them however long it takes to do so. 2nd person just has to wait unlie the 1st person completly served. in paralalle. absolultly nessasary for peroments.
// make sure we don't give people files that they shouldn't have assess to.

// cloud computing: services/mail app for your network keep running a hardware of a super/server mahine fails. virtual computer is a component of cloud computing.
// when you use those web apps, you are acualy assesing files and application (/services/process/for-doing-there-job/tersforming) that are setting on another super/server mahine. through broser the connection is happending. saparating application from hardware. you're just essesing the outcome of an event.
// application that can be clustered, growable, growing, sisuated in a group. like databases. means mysql cluster
// clustering of computing means connecting all the servers application & raplacation will happed. servers sitting in a cluster. multitple server into a cluster. this is done for load-balance (routing) and redundancy (fails).
// want to asses database, they will hit the cluster. cluster will desite which server this person should dereacted to.
// if already has enough connections, the hardware is already being spiked out. now re-routed/re-deracted/load-balancing in the cluster.
// within the application you will create a configaration to create the cluster and create replication means a connection, talk to each other or exchange information. changes will send out.
// all the data is stored is best you put it in one of these cloud environments
// application servers, app screen is shared and all the key and mouse strokes send to the application server. and application server would send back pictures on the secreen. those are not web/php/html applications. application is acually install on the server as if it's install there're computer.
// instend of buying server hardware on your own, just create a instance of your OS in our verusal enverment and then use it for what ever you need. ec2 is just a versual enverment. using claster, using versulization software. versulliation means 1 server mulipule OS, if 1 OS faild other OS resives the data through pain management software for hapaviser/OS.
// hapaviser is a OS type thing. and the management software can create mulpule OS and can tersfer data bettween them and it can turn on and off the pysical server and move, load-balance as need be.
// hosted instands: diffent server location data tersfer. edge location is crazy, paying for the tersfers, small data is find and even better. (diffent server location means all of the cloud computing infastructure, setting within on the internet) ( data means server images they have application & services already installed and you put them amazon, hosted solution means people already created those services)
// hosted solutions: you don't have to worry about buying hardware, you don't have to worry about power supplies dying, some use web apps, any pysical application/services, some use database/services, some use email/services means buying mircrosoft exchange server, client access licenses. you can assess them in the web with installing config anything.
// you set up a cluster of database servers in virtual enverionment, all setting in your server room, that's called private cloud. they're using cloud computing.
// tersfering things, application has been septrate form OS and from the hardware.
// network connections ar pysical hardward ar capablity grow oesa bola there is something call web. with cloud computing a lot more data goes over the network than normal computing environments. all processing happed somewhere alse and result of that processing get sent to your individual computer. once you get into 1000 of computer on a network, all pushing and pullling. extra increased network bandwidth matters. what's you upload/send and downlaod/changes speed is?
// to make sure firewall settings are perfect, to set up root polices for 30 computers on there network etc. who are facing the internet. all the date is secure. if it's connected to the internet in any way or sape or from the hackers can get to it. espacaly small companyes, fire, flood, stole.

// android ISO app is really just the interface, native interaction. data pulling is the same. in database every state you're storing, every events you're storing.

// webassabley

// How to create server room: intro to networking: https://www.youtube.com/playlist?list=PLJcaPjxegjBW3oTHL02TjEw0d6nvFK_cF
// not really: network scaning web appllication: a cron job - sadual nmap to be run in every 10 min, scrip fires off, nmap ar output, the information that it finds into a text file or in to xml file then I use php script in order to parse that xml file, dump the data into a database then I'm able to create a application around that. https://www.youtube.com/watch?v=15bhy5wWuZ4&list=PLJcaPjxegjBVMGKrlf26DgrrWDbZs4OrG and for hacking/security: https://www.youtube.com/watch?v=_mHwudEOt8c&list=PLJcaPjxegjBUIkqq4aC-elec9HH37L0K5

// later update cloud computing: https://youtube.com/playlist?list=PLJcaPjxegjBWWtJHeKzGTTa5JF4UrO2o_&si=aE7g5aLh4sp1IgKx

// how to store data in the databse: programming: https://www.youtube.com/watch?v=0yw-z6f7Mb4 -> user when sending data, could be hack by thrid party and get the data. user machine that is connected to the internat means it can hacked.

// the problem is exicting service, exicting inferstuture, exicting software ka ki va ba improve/mainan kora jae.
///////////////////////////////////////////////////////////////////////////////////////////////////////////////
// I have nothing to do.
// TCP/IP this is the protocal, this is the language that allows systems to talk to each other. more you understand how systems talk to each other, the better it is for actully designing and build your architecture. You need to know full stack
// administer you need to setup you own database server.
// expensive on wild infrastructure is data storage and bandwidth. like aws 9 cents per gig in tersfer.
// how do you connect billion devices and make any of them talk to each other. layer 3 networking, to use routable networks, routable protocols.
// there are mulipule protocol inside TCP/IP protocol.
// IP is the addressing protocol
// ICMP is the internet communication message protocol - ping trace route
// TCP - Transmission protocal, once the computer have found each other, how 2 device actually communicate with each other
// IP adreess is the address to the building, inside that port are particular services, mail room - SMTP port 25, send packages FTP port 20, HTTP port 80, HTTPS port 443, SSH port - 22.
// web broser defual to port 80, so in project we have to use other like 8080 - 127.0.0.1 that call loop back address, internal adress of your network card.

fn main() {
    println!("Hello, world!");
}
