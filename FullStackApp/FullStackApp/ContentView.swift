//
//  ContentView.swift
//  FullStackApp
//
//  Created by Giselle Mendoza on 11/2/24.
//

import SwiftUI

struct ContentView: View {
    @State var ble = BLE()
    var body: some View {
        ZStack{
            NavigationView {
                List {
                    Toggle(isOn: $ble.led) {
                        Text("LED")
                    }
                }
                .navigationTitle("FullStack")
                .disabled(!ble.loaded)
            }
            
            VStack {
                ProgressView()
                    .padding()
                Text(ble.connected ? "Connected. Loading..." : "Looking for device...")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity)
            .background(.background)
            .opacity(ble.loaded ? 0 : 1)
        }
        .animation(.default, value: ble.loaded)
    }
}

#Preview {
    ContentView()
}
