Copy-Item -Path "release-dlls/*.dll" -Destination "." -Force
.\bin\Release\MqttTester.exe
