#include <iostream>
#include <chrono>
#include <mqtt/async_client.h>
#include <thread>
#include <atomic>

std::atomic<bool> running { false };

class callback : public virtual mqtt::callback
{
    mqtt::async_client& client;

    void connected(const std::string& cause) override {
        std::cout << "Connected to server: " << cause << "\n";
        client.subscribe("control", 1);
    }

    void message_arrived(mqtt::const_message_ptr msg) override {
        std::string topic = msg->get_topic();
        std::string payload = msg->to_string();
        if (topic == "control") {
            if (payload == "start") running.store(true);
            else if (payload == "stop") running.store(false);
        }
    }
public:
    callback(mqtt::async_client& client) : client(client) { }
};

std::string addr = "tcp://localhost:6788";

int main()
{
    mqtt::create_options opts(MQTTVERSION_3_1);
    mqtt::async_client client(addr, "publisher", opts);

    auto options = mqtt::connect_options();
    options.set_mqtt_version(MQTTVERSION_3_1);

    std::thread producerThread([&]() {
        size_t index;
        std::string topic = "data";
        for (;;) {
            index = 0;
            while(running.load()) {
                std::string payload = std::to_string(index);
                mqtt::message_ptr msg = mqtt::make_message(topic, payload);
                msg->set_qos(1);
                client.publish(msg);
                index++;
                std::this_thread::sleep_for(std::chrono::milliseconds(20));
            }
            std::this_thread::sleep_for(std::chrono::milliseconds(20));
        }
        });

    callback cb(client);
    client.set_callback(cb);
    mqtt::token_ptr tok = client.connect(options);
    mqtt::connect_response resp = tok->get_connect_response();
   
    for (;;) {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }

    producerThread.detach();
    return 0;
}