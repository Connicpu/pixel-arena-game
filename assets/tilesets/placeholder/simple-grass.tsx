<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.2" tiledversion="1.2.2" name="simple-grass" tilewidth="16" tileheight="16" spacing="1" tilecount="128" columns="16">
 <image source="simple-grass.png" width="271" height="135"/>
 <tile id="0" type="a">
  <properties>
   <property name="flags" value="NULL"/>
  </properties>
 </tile>
 <tile id="1">
  <properties>
   <property name="flags" value="NULL"/>
  </properties>
 </tile>
 <tile id="5">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="3" height="16">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
   <object id="2" x="3" y="13" width="13" height="3">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="6">
  <objectgroup draworder="index">
   <object id="1" x="0" y="13" width="16" height="3">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="7">
  <objectgroup draworder="index">
   <object id="1" x="0" y="13" width="3" height="3">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
   <object id="2" x="13" y="13" width="3" height="3">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
   <object id="3" x="3" y="13" width="10" height="3">
    <properties>
     <property name="flags" value="LADDER|CLIFF"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="8">
  <objectgroup draworder="index">
   <object id="1" x="0" y="13" width="13" height="3">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
   <object id="2" x="13" y="0" width="3" height="16">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="21">
  <objectgroup draworder="index">
   <object id="1" x="2" y="0" width="14" height="16">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="22">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="16" height="16">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="23">
  <objectgroup draworder="index">
   <object id="1" x="3" y="0" width="10" height="16">
    <properties>
     <property name="flags" value="LADDER"/>
    </properties>
   </object>
   <object id="2" x="0" y="0" width="3" height="16">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
   <object id="3" x="13" y="0" width="3" height="16">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="24">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="14" height="16"/>
  </objectgroup>
 </tile>
 <tile id="37">
  <objectgroup draworder="index">
   <object id="1" x="2" y="0" width="14" height="14">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="38">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="16" height="14">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="39">
  <objectgroup draworder="index">
   <object id="1" x="3" y="0" width="10" height="14">
    <properties>
     <property name="flags" value="LADDER"/>
    </properties>
   </object>
   <object id="2" x="0" y="0" width="3" height="14">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
   <object id="5" x="13" y="0" width="3" height="14">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="40">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="14" height="14">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="49">
  <objectgroup draworder="index">
   <object id="1" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|RIGHT"/>
    </properties>
    <polyline points="0,0 8,0"/>
   </object>
   <object id="2" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|LEFT"/>
    </properties>
    <polyline points="0,0 -8,0"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="51">
  <objectgroup draworder="index">
   <object id="1" x="0" y="12" width="4" height="4">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
   <object id="2" x="12" y="12" width="4" height="4">
    <properties>
     <property name="flags" value="CLIFF"/>
    </properties>
   </object>
   <object id="3" x="4" y="12" width="8" height="4">
    <properties>
     <property name="flags" value="LADDER|CLIFF"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="53">
  <objectgroup draworder="index">
   <object id="1" x="2" y="0" width="14" height="8">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
   <object id="2" x="2" y="8" width="14" height="8">
    <properties>
     <property name="flags" value="WALL|VOID"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="54">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="16" height="8">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
   <object id="2" x="0" y="8" width="16" height="8">
    <properties>
     <property name="flags" value="WALL|VOID"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="55">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="3" height="8">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
   <object id="2" x="13" y="0" width="3" height="8">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
   <object id="3" x="0" y="8" width="3" height="8">
    <properties>
     <property name="flags" value="WALL|VOID"/>
    </properties>
   </object>
   <object id="4" x="13" y="8" width="3" height="8">
    <properties>
     <property name="flags" value="WALL|VOID"/>
    </properties>
   </object>
   <object id="5" x="3" y="0" width="10" height="8">
    <properties>
     <property name="flags" value="LADDER"/>
    </properties>
   </object>
   <object id="6" x="3" y="8" width="10" height="8">
    <properties>
     <property name="flags" value="LADDER|VOID"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="56">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="14" height="8">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
   <object id="2" x="0" y="8" width="14" height="8">
    <properties>
     <property name="flags" value="WALL|VOID"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="64">
  <objectgroup draworder="index">
   <object id="1" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|RIGHT"/>
    </properties>
    <polyline points="0,0 8,0"/>
   </object>
   <object id="2" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|DOWN"/>
    </properties>
    <polyline points="0,0 0,8"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="65">
  <objectgroup draworder="index">
   <object id="2" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|LEFT"/>
    </properties>
    <polyline points="0,0 -8,0"/>
   </object>
   <object id="3" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|DOWN"/>
    </properties>
    <polyline points="0,0 0,8"/>
   </object>
   <object id="4" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|RIGHT"/>
    </properties>
    <polyline points="0,0 8,0"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="66">
  <objectgroup draworder="index">
   <object id="1" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|LEFT"/>
    </properties>
    <polyline points="0,0 -8,0"/>
   </object>
   <object id="2" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|DOWN"/>
    </properties>
    <polyline points="0,0 0,8"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="83">
  <objectgroup draworder="index">
   <object id="1" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|UP"/>
    </properties>
    <polyline points="0,0 0,-8"/>
   </object>
   <object id="3" x="8" y="8">
    <properties>
     <property name="flags" value="PATH|DOWN"/>
    </properties>
    <polyline points="0,0 0,8"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="112">
  <objectgroup draworder="index">
   <object id="1" x="2" y="0" width="14" height="16">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
 <tile id="113">
  <objectgroup draworder="index">
   <object id="1" x="0" y="0" width="14" height="16">
    <properties>
     <property name="flags" value="WALL"/>
    </properties>
   </object>
  </objectgroup>
 </tile>
</tileset>
