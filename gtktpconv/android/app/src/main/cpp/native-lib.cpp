#include <jni.h>

#include "lib.hxx"

extern "C" jobjectArray
Java_hu_vmiklos_tpconv_MainActivity_getUnitNames(JNIEnv* env,
                                                 jobject /* this */)
{
    std::initializer_list<std::string> unitNames = tpconv::getUnitNames();

    jobjectArray array = env->NewObjectArray(unitNames.size(),
                                             env->FindClass("java/lang/String"),
                                             env->NewStringUTF(""));
    size_t i = 0;
    for (const auto& unit : unitNames)
    {
        env->SetObjectArrayElement(array, i, env->NewStringUTF(unit.c_str()));
        ++i;
    }

    return array;
}

extern "C" jdouble Java_hu_vmiklos_tpconv_MainActivity_convert(
    JNIEnv* env, jobject /* this */, jdouble amount, jint fromInt, jint toInt)
{
    auto from = static_cast<tpconv::ConversionUnit>(fromInt);
    auto to = static_cast<tpconv::ConversionUnit>(toInt);
    return tpconv::convert(amount, from, to);
}
