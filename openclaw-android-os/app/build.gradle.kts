plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
    id("org.jetbrains.kotlin.plugin.serialization")
    id("com.google.dagger.hilt.android")
    id("com.google.devtools.ksp")
}

android {
    namespace = "com.openclaw.callingnode"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.openclaw.callingnode"
        minSdk = 28
        targetSdk = 35
        versionCode = 1
        versionName = "1.0.0-alpha"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

        // Build config fields for API keys (set via local.properties or CI)
        buildConfigField("String", "VAPI_API_KEY", "\"${project.findProperty("VAPI_API_KEY") ?: ""}\"")
        buildConfigField("String", "VAPI_BASE_URL", "\"https://api.vapi.ai\"")
        buildConfigField("String", "WHATSAPP_ACCESS_TOKEN", "\"${project.findProperty("WHATSAPP_ACCESS_TOKEN") ?: ""}\"")
        buildConfigField("String", "WHATSAPP_PHONE_NUMBER_ID", "\"${project.findProperty("WHATSAPP_PHONE_NUMBER_ID") ?: ""}\"")
        buildConfigField("String", "WHATSAPP_BUSINESS_ACCOUNT_ID", "\"${project.findProperty("WHATSAPP_BUSINESS_ACCOUNT_ID") ?: ""}\"")
        buildConfigField("String", "OPENCLAW_GATEWAY_URL", "\"${project.findProperty("OPENCLAW_GATEWAY_URL") ?: "ws://localhost:18789"}\"")
        buildConfigField("String", "OPENCLAW_GATEWAY_TOKEN", "\"${project.findProperty("OPENCLAW_GATEWAY_TOKEN") ?: ""}\"")
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isMinifyEnabled = false
            applicationIdSuffix = ".debug"
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        compose = true
        buildConfig = true
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    // ── Core Android ──
    implementation("androidx.core:core-ktx:1.15.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.8.7")
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.8.7")
    implementation("androidx.activity:activity-compose:1.9.3")

    // ── Jetpack Compose ──
    implementation(platform("androidx.compose:compose-bom:2024.12.01"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.material:material-icons-extended")
    implementation("androidx.navigation:navigation-compose:2.8.5")
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")

    // ── Dependency Injection (Hilt) ──
    implementation("com.google.dagger:hilt-android:2.53.1")
    ksp("com.google.dagger:hilt-compiler:2.53.1")
    implementation("androidx.hilt:hilt-navigation-compose:1.2.0")

    // ── Networking ──
    implementation("com.squareup.retrofit2:retrofit:2.11.0")
    implementation("com.squareup.retrofit2:converter-kotlinx-serialization:2.11.0")
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.squareup.okhttp3:logging-interceptor:4.12.0")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.7.3")

    // ── WebSocket (for Gateway connection) ──
    implementation("com.squareup.okhttp3:okhttp:4.12.0")

    // ── WebRTC (for WhatsApp calling) ──
    implementation("io.getstream:stream-webrtc-android:1.3.1")

    // ── DataStore (for preferences/config) ──
    implementation("androidx.datastore:datastore-preferences:1.1.2")

    // ── WorkManager (for background tasks) ──
    implementation("androidx.work:work-runtime-ktx:2.10.0")

    // ── Room Database (for call logs) ──
    implementation("androidx.room:room-runtime:2.6.1")
    implementation("androidx.room:room-ktx:2.6.1")
    ksp("androidx.room:room-compiler:2.6.1")

    // ── Coroutines ──
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.9.0")

    // ── Telephony / TelecomManager ──
    implementation("androidx.core:core-telecom:1.0.0-alpha04")

    // ── Audio Processing ──
    implementation("com.google.android.exoplayer:exoplayer-core:2.19.1")

    // ── Logging ──
    implementation("com.jakewharton.timber:timber:5.0.1")

    // ── Testing ──
    testImplementation("junit:junit:4.13.2")
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.9.0")
    testImplementation("io.mockk:mockk:1.13.13")
    androidTestImplementation("androidx.test.ext:junit:1.2.1")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.6.1")
    androidTestImplementation(platform("androidx.compose:compose-bom:2024.12.01"))
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
}
