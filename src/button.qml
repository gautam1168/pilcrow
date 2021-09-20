import QtQuick 2.8

Item {
    id: myButton_Symbol
    width: 211
    height: 211
    state: "default"
    
    Image {
        id: myButton_Symbol_default
        x: 0
        y: 0
        source: "assets/myButton_Symbol_default.png"
    }
    
    Image {
        id: myButton_Symbol_hover
        x: 0
        y: 0
        source: "assets/myButton_Symbol_hover.png"
    }
    
    Image {
        id: myButton_Symbol_pressed
        x: 0
        y: 0
        source: "assets/myButton_Symbol_pressed.png"
    }
    
    MouseArea {
        id: myButton_Symbol_MouseArea
        x: 0
        y: 0
        width: 211
        height: 211
        hoverEnabled: true
    }
    
    states: [
        
        State {
            name: "default"
            when: !myButton_Symbol_MouseArea.containsMouse
                  && !myButton_Symbol_MouseArea.pressed
            
            PropertyChanges {
                target: myButton_Symbol_hover
                visible: false
            }
            
            PropertyChanges {
                target: myButton_Symbol_pressed
                visible: false
            }
        },
        
        State {
            name: "hover"
            when: myButton_Symbol_MouseArea.containsMouse
                  && !myButton_Symbol_MouseArea.pressed
            
            PropertyChanges {
                target: myButton_Symbol_pressed
                visible: false
            }
            
            PropertyChanges {
                target: myButton_Symbol_default
                visible: false
            }
        },
        
        State {
            name: "pressed"
            when: myButton_Symbol_MouseArea.pressed
            
            PropertyChanges {
                target: myButton_Symbol_default
                visible: false
            }
            
            PropertyChanges {
                target: myButton_Symbol_hover
                visible: false
            }
        }
    ]
}