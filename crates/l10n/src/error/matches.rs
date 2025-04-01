pub const fn map(lang: &[u8], map_name: &[u8], key: &[u8]) -> &'static str {
  match (lang, map_name, key) {
    (b"af", b"error", b"text-not-found") => {
      r#####"Geen gelokaliseerde teks gevind nie"#####
    }
    (b"am", b"error", b"text-not-found") => r#####"የተካሄደው ጽሑፍ አልተገኘም"#####,
    (b"ar", b"error", b"text-not-found") => r#####"لم يتم العثور على نص محلي"#####,
    (b"az", b"error", b"text-not-found") => r#####"Yerli mətn tapılmadı"#####,
    (b"be", b"error", b"text-not-found") => {
      r#####"Ніякага лакалізаванага тэксту не знойдзена"#####
    }
    (b"bg", b"error", b"text-not-found") => {
      r#####"Не е намерен локализиран текст"#####
    }
    (b"bn", b"error", b"text-not-found") => r#####"স্থানীয় কোনও পাঠ্য পাওয়া যায় নি"#####,
    (b"bs", b"error", b"text-not-found") => {
      r#####"Nije pronađen lokalizirani tekst"#####
    }
    (b"ca", b"error", b"text-not-found") => {
      r#####"No s'ha trobat cap text localitzat"#####
    }
    (b"ceb", b"error", b"text-not-found") => {
      r#####"Wala'y nakit-an nga lokal nga teksto"#####
    }
    (b"co", b"error", b"text-not-found") => {
      r#####"Nisun testu localizatu truvatu"#####
    }
    (b"cs", b"error", b"text-not-found") => r#####"Žádný lokalizovaný text"#####,
    (b"cy", b"error", b"text-not-found") => {
      r#####"Ni ddarganfuwyd testun lleol"#####
    }
    (b"da", b"error", b"text-not-found") => {
      r#####"Ingen lokaliseret tekst fundet"#####
    }
    (b"de", b"error", b"text-not-found") => {
      r#####"Kein lokalisierter Text gefunden"#####
    }
    (b"el", b"error", b"text-not-found") => {
      r#####"Δεν βρέθηκε κανένα τοπικό κείμενο"#####
    }
    (b"en", b"error", b"text-not-found") => r#####"No localized text found"#####,
    (b"en-GB", b"error", b"text-not-found") => r#####"No localised text found"#####,
    (b"eo", b"error", b"text-not-found") => {
      r#####"Neniu lokalizita teksto trovita"#####
    }
    (b"es", b"error", b"text-not-found") => {
      r#####"No se encontró texto localizado"#####
    }
    (b"et", b"error", b"text-not-found") => {
      r#####"Lokaliseeritud teksti ei leitud"#####
    }
    (b"eu", b"error", b"text-not-found") => r#####"Ez da testurik aurkitu"#####,
    (b"fa", b"error", b"text-not-found") => r#####"هیچ متن محلی یافت نشده است"#####,
    (b"fi", b"error", b"text-not-found") => {
      r#####"Paikallista tekstiä ei löydy"#####
    }
    (b"fil", b"error", b"text-not-found") => {
      r#####"Walang nahanap na naisalokal na teksto"#####
    }
    (b"fr", b"error", b"text-not-found") => r#####"Aucun texte localisé trouvé"#####,
    (b"fy", b"error", b"text-not-found") => r#####"Gjin lokaliseare tekst fûn"#####,
    (b"ga", b"error", b"text-not-found") => {
      r#####"Ní bhfuarthas aon téacs logánta"#####
    }
    (b"gd", b"error", b"text-not-found") => {
      r#####"Cha deach teacsa ionadail a lorg"#####
    }
    (b"gl", b"error", b"text-not-found") => {
      r#####"Non se atopou texto localizado"#####
    }
    (b"gu", b"error", b"text-not-found") => r#####"કોઈ સ્થાનિક લખાણ મળ્યું નથી"#####,
    (b"ha", b"error", b"text-not-found") => {
      r#####"Ba a sami rubutu mara nauyi ba"#####
    }
    (b"haw", b"error", b"text-not-found") => r#####"ʻAʻohe mea iʻikeʻia"#####,
    (b"he", b"error", b"text-not-found") => r#####"לא נמצא טקסט מקומי"#####,
    (b"hi", b"error", b"text-not-found") => r#####"कोई स्थानीय पाठ नहीं मिला"#####,
    (b"hr", b"error", b"text-not-found") => {
      r#####"Nije pronađen lokalizirani tekst"#####
    }
    (b"ht", b"error", b"text-not-found") => {
      r#####"Pa gen tèks lokalize yo te jwenn"#####
    }
    (b"hu", b"error", b"text-not-found") => {
      r#####"Nem található lokalizált szöveg"#####
    }
    (b"hy", b"error", b"text-not-found") => r#####"Տեղայնացված տեքստ չի գտնվել"#####,
    (b"id", b"error", b"text-not-found") => {
      r#####"Tidak ada teks lokal yang ditemukan"#####
    }
    (b"ig", b"error", b"text-not-found") => r#####"Enweghị ederede edepụtara"#####,
    (b"is", b"error", b"text-not-found") => {
      r#####"Enginn staðbundinn texti fannst"#####
    }
    (b"it", b"error", b"text-not-found") => {
      r#####"Nessun testo localizzato trovato"#####
    }
    (b"ja", b"error", b"text-not-found") => {
      r#####"本地化文書未検出(ローカライズド・テキスト・ノット・ファウンド)"#####
    }
    (b"ja-Latn-JP", b"error", b"text-not-found") => {
      r#####"Rōkarāizu sareta tekisuto wa mitsukarimasen"#####
    }
    (b"jw", b"error", b"text-not-found") => r#####"Ora ditemokake teks lokal"#####,
    (b"ka", b"error", b"text-not-found") => {
      r#####"ლოკალიზებული ტექსტი ვერ მოიძებნა"#####
    }
    (b"kk", b"error", b"text-not-found") => {
      r#####"Локализацияланған мәтін табылған жоқ"#####
    }
    (b"km", b"error", b"text-not-found") => r#####"រកមិនឃើញអត្ថបទដែលបានធ្វើមូលដ្ឋានីយកម្ម"#####,
    (b"kn", b"error", b"text-not-found") => r#####"ಯಾವುದೇ ಸ್ಥಳೀಯ ಪಠ್ಯ ಕಂಡುಬಂದಿಲ್ಲ"#####,
    (b"ko", b"error", b"text-not-found") => r#####"현지화 된 텍스트가 없습니다"#####,
    (b"ku", b"error", b"text-not-found") => r#####"Nivîsek herêmî nehat dîtin"#####,
    (b"ky", b"error", b"text-not-found") => {
      r#####"Локалдаштырылган текст табылган жок"#####
    }
    (b"la", b"error", b"text-not-found") => {
      r#####"Non localized illud invenitur"#####
    }
    (b"lb", b"error", b"text-not-found") => r#####"Kee lokalen Text fonnt"#####,
    (b"lo", b"error", b"text-not-found") => r#####"ບໍ່ພົບຂໍ້ຄວາມທ້ອງຖິ່ນ"#####,
    (b"lt", b"error", b"text-not-found") => r#####"Nerasta lokalizuoto teksto"#####,
    (b"lv", b"error", b"text-not-found") => {
      r#####"Nav atrasts lokalizēts teksts"#####
    }
    (b"lzh", b"error", b"text-not-found") => r#####"方俗篇章闕如，宜補遺軼。"#####,
    (b"mg", b"error", b"text-not-found") => {
      r#####"Tsy nisy lahatsoratra hita teo an-toerana hita"#####
    }
    (b"mi", b"error", b"text-not-found") => {
      r#####"Kaore i kitea he tuhinga kua kitea"#####
    }
    (b"mk", b"error", b"text-not-found") => {
      r#####"Не е пронајден локализиран текст"#####
    }
    (b"ml", b"error", b"text-not-found") => {
      r#####"പ്രാദേശികവൽക്കരിച്ച വാചകം കണ്ടെത്തിയില്ല"#####
    }
    (b"mn", b"error", b"text-not-found") => {
      r#####"Орон нутгийн текст олдсонгүй"#####
    }
    (b"mr", b"error", b"text-not-found") => {
      r#####"कोणताही स्थानिक मजकूर सापडला नाही"#####
    }
    (b"ms", b"error", b"text-not-found") => {
      r#####"Tidak ada teks setempat yang dijumpai"#####
    }
    (b"mt", b"error", b"text-not-found") => {
      r#####"Ma nstab l-ebda test lokalizzat"#####
    }
    (b"my", b"error", b"text-not-found") => r#####"အဘယ်သူမျှမဒေသဆိုင်ရာစာသားကိုရှာမတွေ့ပါ"#####,
    (b"ne", b"error", b"text-not-found") => r#####"कुनै स्थानीय पद फेला परेन"#####,
    (b"nl", b"error", b"text-not-found") => {
      r#####"Geen gelokaliseerde tekst gevonden"#####
    }
    (b"no", b"error", b"text-not-found") => {
      r#####"Ingen lokalisert tekst funnet"#####
    }
    (b"ny", b"error", b"text-not-found") => {
      r#####"Palibe zolemba zomwe zapezeka"#####
    }
    (b"or", b"error", b"text-not-found") => {
      r#####"କ local ଣସି ସ୍ଥାନୀୟ ପାଠ୍ୟ ମିଳିଲା ନାହିଁ |"#####
    }
    (b"pa", b"error", b"text-not-found") => r#####"ਕੋਈ ਸਥਾਨਕ ਨਹੀਂ ਲੱਭਿਆ"#####,
    (b"pl", b"error", b"text-not-found") => {
      r#####"Nie znaleziono zlokalizowanego tekstu"#####
    }
    (b"ps", b"error", b"text-not-found") => r#####"هیڅ ځایی شوی متن وموندل شو"#####,
    (b"pt", b"error", b"text-not-found") => {
      r#####"Nenhum texto localizado encontrado"#####
    }
    (b"ro", b"error", b"text-not-found") => {
      r#####"Nu a fost găsit niciun text localizat"#####
    }
    (b"ru", b"error", b"text-not-found") => {
      r#####"Локализованный текст не найден"#####
    }
    (b"sd", b"error", b"text-not-found") => r#####"مقامي متن نه مليو"#####,
    (b"si", b"error", b"text-not-found") => r#####"දේශීයකරණය කළ පෙළක් හමු නොවීය"#####,
    (b"sk", b"error", b"text-not-found") => {
      r#####"Nenašiel sa žiadny lokalizovaný text"#####
    }
    (b"sl", b"error", b"text-not-found") => {
      r#####"Ni bilo najdenega lokaliziranega besedila"#####
    }
    (b"sm", b"error", b"text-not-found") => {
      r#####"Leai se tusitusiga i le lotoifale maua"#####
    }
    (b"sn", b"error", b"text-not-found") => {
      r#####"Hapana zvinyorwa zvemukati zvinowanikwa"#####
    }
    (b"so", b"error", b"text-not-found") => {
      r#####"Ma jiro qoraal maxalli ah oo la helay"#####
    }
    (b"sq", b"error", b"text-not-found") => {
      r#####"Asnjë tekst i lokalizuar nuk u gjet"#####
    }
    (b"sr", b"error", b"text-not-found") => {
      r#####"Није пронађен ниједан локализовани текст"#####
    }
    (b"st", b"error", b"text-not-found") => {
      r#####"Ha ho na sengoloa sa lehae se fumanoeng"#####
    }
    (b"su", b"error", b"text-not-found") => {
      r#####"Henteu aya téks anu dilereskeun"#####
    }
    (b"sv", b"error", b"text-not-found") => {
      r#####"Ingen lokaliserad text hittades"#####
    }
    (b"sw", b"error", b"text-not-found") => {
      r#####"Hakuna maandishi ya ndani yaliyopatikana"#####
    }
    (b"ta", b"error", b"text-not-found") => {
      r#####"உள்ளூர்மயமாக்கப்பட்ட உரை எதுவும் கிடைக்கவில்லை"#####
    }
    (b"te", b"error", b"text-not-found") => r#####"స్థానికీకరించిన వచనం కనుగొనబడలేదు"#####,
    (b"tg", b"error", b"text-not-found") => {
      r#####"Ягон матни маҳаллӣ ёфт нашуд"#####
    }
    (b"th", b"error", b"text-not-found") => r#####"ไม่พบข้อความที่แปลเป็นภาษาท้องถิ่น"#####,
    (b"tr", b"error", b"text-not-found") => {
      r#####"Yerelleştirilmiş metin bulunamadı"#####
    }
    (b"ug", b"error", b"text-not-found") => {
      r#####"يەرلىكلەشتۈرۈلگەن تېكىست تېپىلمىدى"#####
    }
    (b"uk", b"error", b"text-not-found") => {
      r#####"Не знайдено локалізованого тексту"#####
    }
    (b"ur", b"error", b"text-not-found") => r#####"کوئی مقامی متن نہیں ملا"#####,
    (b"uz", b"error", b"text-not-found") => r#####"Mahalliy matn topilmadi"#####,
    (b"vi", b"error", b"text-not-found") => {
      r#####"Không tìm thấy văn bản bản địa hóa"#####
    }
    (b"xh", b"error", b"text-not-found") => {
      r#####"Akukho sicatshulwa sendawo sifunyenwe"#####
    }
    (b"yi", b"error", b"text-not-found") => {
      r#####"קיין לאָוקאַלייזד טעקסט געפֿונען"#####
    }
    (b"yo", b"error", b"text-not-found") => {
      r#####"Ko si ọrọ ti agbegbe ti a rii"#####
    }
    (b"zh", b"error", b"text-not-found") => r#####"未找到本地化文本"#####,
    (b"zh-Hant", b"error", b"text-not-found") => r#####"沒有找到本地化文本"#####,
    (b"zh-Latn-CN", b"error", b"text-not-found") => {
      r#####"MeiYou ZhaoDao BenDiHua WenBen"#####
    }
    (b"zu", b"error", b"text-not-found") => {
      r#####"Awukho umbhalo wasendaweni otholakala"#####
    }
    _ => "",
  }
}
