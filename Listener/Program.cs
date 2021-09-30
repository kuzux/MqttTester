using System;
using System.Text;
using System.Threading;
using MQTTnet;
using MQTTnet.Client;
using MQTTnet.Client.Options;
using MQTTnet.Client.Subscribing;

namespace Listener
{
    class Program
    {
        static void Main(string[] args)
        {
            var factory = new MqttFactory();
            var client = factory.CreateMqttClient();
            var options = new MqttClientOptionsBuilder()
                .WithTcpServer("127.0.0.1", 6788)
                .WithClientId("Subscriber")
                .Build();
            client.UseConnectedHandler(async e => {
                // What I needed to do for establishing connection:
                // Install mosquitto
                // Add these lines to mosquitto.conf
                // listener 6788 127.0.0.1
                // allow_anonymous true
                // Allow port 6788 on windows firewall
                // Run .\mosquitto.exe -c mosquitto.conf (in mosquitto directory)
                Console.WriteLine("Connected to mqtt server");

                await client.SubscribeAsync(new MqttClientSubscribeOptionsBuilder().WithTopicFilter("data").Build());
                await client.PublishAsync("control", "start");
            });
            client.UseApplicationMessageReceivedHandler(e =>
            {
                Console.WriteLine($"Received in {e.ApplicationMessage.Topic}: {Encoding.UTF8.GetString(e.ApplicationMessage.Payload)}");
            });

            client.ConnectAsync(options, CancellationToken.None);
            Thread.Sleep(20000);
            client.PublishAsync("control", "stop"); 
            Thread.Sleep(50);
        }
    }
}
