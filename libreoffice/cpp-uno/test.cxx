/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
#include <cppuhelper/bootstrap.hxx>
#include <rtl/process.h>
#include <sal/main.h>

#include <com/sun/star/beans/XPropertySet.hpp>
#include <com/sun/star/bridge/XUnoUrlResolver.hpp>
#include <com/sun/star/lang/XMultiComponentFactory.hpp>
#include <com/sun/star/frame/XComponentLoader.hpp>

using namespace com::sun::star;
using ::rtl::OUString;

SAL_IMPLEMENT_MAIN_WITH_ARGS(argc, argv)
{
    // Extract parameters.
    OUString sConnectionString("uno:socket,host=localhost,port=2083;urp;StarOffice.ServiceManager"), sDocUrl;
    sal_Int32 nCount(rtl_getAppCommandArgCount());
    if (nCount < 1)
    {
        SAL_DEBUG("usage: test -env:URE_MORE_TYPES=<office_types_rdb_url> <file_url> [<uno_connection_url>]");
        SAL_DEBUG("example: test -env:URE_MORE_TYPES=\"file:///.../program/offapi.rdb\" \"file:///e:/temp/test.odt\"");
        exit(1);
    }
    rtl_getAppCommandArg(0, &sDocUrl.pData);
    if (nCount == 2)
        rtl_getAppCommandArg(1, &sConnectionString.pData);

    // Initialize UNO: result is the xComponentContext what we need to create any service.
    uno::Reference<uno::XComponentContext> xComponentContext;
    try
    {
        xComponentContext.set(cppu::defaultBootstrap_InitialComponentContext());
    }
    catch (uno::Exception& e)
    {
        SAL_DEBUG("cppu::defaultBootstrap_InitialComponentContext() failed: '" << e.Message << "'");
        exit(1);
    }
    uno::Reference<lang::XMultiComponentFactory> xMultiComponentFactory(xComponentContext->getServiceManager());
    uno::Reference<bridge::XUnoUrlResolver> xResolver(xMultiComponentFactory->createInstanceWithContext("com.sun.star.bridge.UnoUrlResolver", xComponentContext), uno::UNO_QUERY);
    uno::Reference<beans::XPropertySet> xPropertySet;
    try
    {
        xPropertySet.set(xResolver->resolve(sConnectionString), uno::UNO_QUERY);
    }
    catch (uno::Exception& e)
    {
        SAL_DEBUG("cannot establish a connection using '" << sConnectionString << "': " << e.Message);
        exit(1);
    }
    xPropertySet->getPropertyValue("DefaultContext") >>= xComponentContext;
    xMultiComponentFactory = xComponentContext->getServiceManager();

    // Load the document: create the frame::Desktop service and load the document.
    uno::Reference<frame::XComponentLoader> xComponentLoader(xMultiComponentFactory->createInstanceWithContext("com.sun.star.frame.Desktop", xComponentContext), uno::UNO_QUERY);
    sal_uInt32 nStartTime = osl_getGlobalTimer();
    uno::Reference<lang::XComponent> xComponent = xComponentLoader->loadComponentFromURL(sDocUrl, "_blank", 0,uno::Sequence<beans::PropertyValue>());
    sal_uInt32 nEndTime = osl_getGlobalTimer();
    SAL_DEBUG("loadComponentFromURL() finished in " << nEndTime - nStartTime << " ms");
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
