import speech_recognition as sr
import pyttsx3
from process_text import process_text

print("HELLOOOO")

botname = "андрей"
text = 'какой-нибудь текст'
tts = pyttsx3.init()
rate = tts.getProperty('rate') #Скорость произношения
tts.setProperty('rate', rate+400)

volume = tts.getProperty('volume') #Громкость голоса
tts.setProperty('volume', volume+0.9)

voices = tts.getProperty('voices')

# Задать голос по умолчанию
tts.setProperty('voice', 'en')

#for voice in voices:
#    print(voice.name)
#    if voice.name == 'russian':
#        tts.setProperty('voice', voice.id)

def say(text):
        tts.say(text)
        tts.runAndWait()
def process_speech(text):
    #if (text.find("слушай " + botname)):
#        say("Привет, что тебе надо?")
#    else:
     say(text)

def record_volume():
    r = sr.Recognizer()
    with sr.Microphone(device_index = 14) as source:
        print('Настраиваюсь.')
        r.adjust_for_ambient_noise(source, duration=0.5) #настройка посторонних шумов
        print('Слушаю...')
        audio = r.listen(source)
    print('Услышала.')
    try:
        query = r.recognize_google(audio, language = 'ru-RU')
        text = query.lower()
        process_speech(text)
    except:
        print('Error')

while True:
    record_volume()
