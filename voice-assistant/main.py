import speech_recognition as sr
from process_text import process_text

print("HELLOOOO")

botname = "андрей"
text = 'какой-нибудь текст'


# Задать голос по умолчанию

#for voice in voices:
#    print(voice.name)
#    if voice.name == 'russian':
#        tts.setProperty('voice', voice.id)

    #if (text.find("слушай " + botname)):
#        say("Привет, что тебе надо?")
#    else:

#for index, name in enumerate(sr.Microphone.list_microphone_names()):
#    print("Microphone with name \"{1}\" found for `Microphone(device_index={0})`".format(index, name))

def record_volume():
    r = sr.Recognizer()
    with sr.Microphone(device_index = 1) as source:
        try:
            print('Настраиваюсь.')
            #r.adjust_for_ambient_noise(source, duration=0.5) #настройка посторонних шумов
            print('Слушаю...')
            audio = r.listen(source)
            print('Услышала.')
            try:
                query = r.recognize_google(audio, language = 'ru-RU')
                text = query.lower()
                print(tex)
            except:
                print('Error')
        except:
            print('Error')
while True:
    record_volume()
