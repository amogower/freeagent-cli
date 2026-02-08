# ═══════════════════════════════════════════════════════════════════════════════
# OpenClaw Calling Node — ProGuard Rules
# ═══════════════════════════════════════════════════════════════════════════════

# ── Kotlin Serialization ──
-keepattributes *Annotation*, InnerClasses
-dontnote kotlinx.serialization.AnnotationsKt

-keepclassmembers class kotlinx.serialization.json.** {
    *** Companion;
}
-keepclasseswithmembers class kotlinx.serialization.json.** {
    kotlinx.serialization.KSerializer serializer(...);
}

-keep,includedescriptorclasses class com.openclaw.callingnode.model.**$$serializer { *; }
-keepclassmembers class com.openclaw.callingnode.model.** {
    *** Companion;
}
-keepclasseswithmembers class com.openclaw.callingnode.model.** {
    kotlinx.serialization.KSerializer serializer(...);
}

-keep,includedescriptorclasses class com.openclaw.callingnode.service.whatsapp.**$$serializer { *; }
-keepclassmembers class com.openclaw.callingnode.service.whatsapp.** {
    *** Companion;
}
-keepclasseswithmembers class com.openclaw.callingnode.service.whatsapp.** {
    kotlinx.serialization.KSerializer serializer(...);
}

-keep,includedescriptorclasses class com.openclaw.callingnode.service.vapi.**$$serializer { *; }
-keepclassmembers class com.openclaw.callingnode.service.vapi.** {
    *** Companion;
}
-keepclasseswithmembers class com.openclaw.callingnode.service.vapi.** {
    kotlinx.serialization.KSerializer serializer(...);
}

# ── Retrofit ──
-keepattributes Signature, InnerClasses, EnclosingMethod
-keepattributes RuntimeVisibleAnnotations, RuntimeVisibleParameterAnnotations
-keepattributes AnnotationDefault
-keepclassmembers,allowshrinking,allowobfuscation interface * {
    @retrofit2.http.* <methods>;
}
-dontwarn org.codehaus.mojo.animal_sniffer.IgnoreJRERequirement
-dontwarn javax.annotation.**
-dontwarn kotlin.Unit
-dontwarn retrofit2.KotlinExtensions
-dontwarn retrofit2.KotlinExtensions$*

# ── OkHttp ──
-dontwarn okhttp3.**
-dontwarn okio.**
-dontwarn javax.annotation.**
-keepnames class okhttp3.internal.publicsuffix.PublicSuffixDatabase

# ── WebRTC ──
-keep class org.webrtc.** { *; }
-dontwarn org.webrtc.**

# ── Hilt / Dagger ──
-dontwarn dagger.**
-keep class dagger.** { *; }
-keep class javax.inject.** { *; }
-keep class * extends dagger.hilt.android.internal.managers.ViewComponentManager$FragmentContextWrapper { *; }

# ── Timber ──
-dontwarn org.jetbrains.annotations.**

# ── General ──
-keepattributes SourceFile,LineNumberTable
-renamesourcefileattribute SourceFile
