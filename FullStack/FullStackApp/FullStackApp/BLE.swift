//
//  BLE.swift
//  FullStackApp
//
//  Created by Giselle Mendoza on 11/2/24.
//
import CoreBluetooth
import os

enum Endpoint {
    case led
}


@Observable
class BLE: NSObject, CBCentralManagerDelegate, CBPeripheralDelegate {
    // more to come...
    private static let LOGGER = Logger(subsystem: Bundle.main.bundleIdentifier!, category: BLE.description())
    
    // expected service UUID
    static let SERVICE_UUID: CBUUID = CBUUID(string: "937312e0-2354-11eb-9f10-fbc30a62cf38")
    // expected characteristic UUIDs
    static let CHARACTERISTIC_MAP: [CBUUID: Endpoint] = [
        CBUUID(string: "957312e0-2354-11eb-9f10-fbc30a62cf38"): .led,
    ]
    
    // the central manager store
    var centralManager: CBCentralManager!

    override init() {
        super.init()
        // attach to the central manager
        centralManager = CBCentralManager(delegate: self, queue: nil)
    }

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        switch central.state {
        // if the central manager is on...
        case .poweredOn:
            // ...start scanning
            startScanning()
        default:
            Self.LOGGER.error("Central manager state is not powered on.")
        }
    }

    func startScanning() {
        Self.LOGGER.info("Scanning...")
        // start scanning for devices advertising the expected service
        centralManager.scanForPeripherals(withServices: [Self.SERVICE_UUID])
    }
    
    func stopScanning() {
        centralManager.stopScan()
    }

    // the peripheral store
    var peripheral: CBPeripheral?
    
    // discovered characteristics
    var characteristics: [Endpoint: CBCharacteristic] = [:]

    // when the central manager discovers a peripheral...
    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String : Any], rssi RSSI: NSNumber) {
        Self.LOGGER.info("Discovered peripheral: \(peripheral).")
            // ...store the peripheral...
            self.peripheral = peripheral
        // ...and connect to it
        centralManager.connect(peripheral)
    }

    // when the central manager connects to a peripheral...
    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        Self.LOGGER.info("Connected to peripheral: \(peripheral).")
        // ...assign this instance as the peripheral delegate...
        self.peripheral!.delegate = self
        // ...and discover the expected services of the peripheral
        self.peripheral!.discoverServices([Self.SERVICE_UUID])
        // no longer need to scan since a connection has been made
        stopScanning()
    }

    // when the peripheral disconnects...
    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: (any Error)?) {
        Self.LOGGER.info("Disconnected from peripheral: \(peripheral).")
        // ...invalidate peripheral and characterstic handles
            self.peripheral = nil
        characteristics = [:]
        // resume scanning to potentially connect again
        startScanning()
    }
    
    // when all services have been discovered...
    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: (any Error)?) {
        let services = peripheral.services!
        Self.LOGGER.info("Discovered services: \(services).")
        // ...for each discovered service...
        for service in services {
            // ...if the service is the service we are looking for...
            if service.uuid == Self.SERVICE_UUID {
                // ...discover all expected characteristics from that service
                peripheral.discoverCharacteristics(Array(Self.CHARACTERISTIC_MAP.keys), for: service)
                break // don't care about other services
            }
        }
    }

    // when all characteristics of a service have been discovered...
    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: (any Error)?) {
        let characteristics = service.characteristics!
        Self.LOGGER.info("Discovered characteristics: \(characteristics).")
        // ...for each discovered characteristic...
        for characteristic in characteristics {
            // ...store into the charactersistics dict...
            // NOTE: all discovered characteristics should be expected so this will never panic
            self.characteristics[Self.CHARACTERISTIC_MAP[characteristic.uuid]!] = characteristic
            // ...and read the value
            peripheral.readValue(for: characteristic)
        }
    }

    // internal led state
    private var led_state = false

    // when a characteristic value has updated...
    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: (any Error)?) {
        Self.LOGGER.info("Value updated for characteristic: \(characteristic).")
        // ...if the characteristic is the LED characteristic...
        if characteristic.uuid == characteristics[.led]!.uuid {
            // ...update the LED state with the new value
                led_state = characteristic.value!.withUnsafeBytes({ ptr in
                    ptr.load(as: Bool.self)
                })
            Self.LOGGER.trace("LED state updated: \(self.led_state).")
        }
    }

    // exposed led state
    var led: Bool {
        // when this property is read, return the internal state
        get {
            led_state
        }
        // when this property is written to, send the new value to the BLE peripheral
        // and update the internal state
        set(newValue) {
            peripheral?.writeValue(Data([newValue ? 1 : 0]), for: characteristics[.led]!, type: .withResponse)
            led_state = newValue
            Self.LOGGER.trace("LED state written: \(newValue).")
        }
    }

    // if the peripheral is not null, it must be connected
    var connected: Bool {
        peripheral != nil
    }
    // if all expected characteristics have been found, discovery is complete
    var loaded: Bool {
        characteristics.count == Self.CHARACTERISTIC_MAP.count
    }


}

